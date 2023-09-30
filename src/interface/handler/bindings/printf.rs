use core::{
    ffi::{CStr, VaListImpl},
    fmt::Display,
    iter::Peekable,
};

use crate::handler::OS_INTERFACE;

fn read_format_parameter(
    chars: &mut Peekable<impl Iterator<Item = char>>,
    args: &mut VaListImpl<'_>,
) -> Option<(usize, bool)> {
    match chars.peek() {
        Some('*') => {
            let param = unsafe { args.arg::<core::ffi::c_int>() as isize };

            chars.next();

            Some((param.unsigned_abs(), param < 0))
        }
        Some('1'..='9') => {
            let mut number = 0;

            loop {
                let Some(&c) = chars.peek() else {
                    break;
                };
                if !c.is_ascii_digit() {
                    break;
                }

                chars.next();

                number *= 10;
                number += c as usize - '0' as usize;
            }

            Some((number, false))
        }

        _ => None,
    }
}

fn read_min_width(
    chars: &mut Peekable<impl Iterator<Item = char>>,
    args: &mut VaListImpl,
    justify_left: &mut bool,
) -> Option<usize> {
    match read_format_parameter(chars, args) {
        Some((result, output_was_negative)) => {
            if output_was_negative {
                *justify_left = true;
            }
            Some(result)
        }
        None => None,
    }
}

fn read_precision(
    chars: &mut Peekable<impl Iterator<Item = char>>,
    args: &mut VaListImpl,
) -> Option<usize> {
    match read_format_parameter(chars, args) {
        Some((result, false)) => Some(result),
        Some((_, true)) => panic!("Invalid printf precision specifier"),
        None => None,
    }
}

struct CFmtConverter<'a, 'b> {
    format: &'a str,
    args: &'b VaListImpl<'a>,
}

impl<'a, 'b> Display for CFmtConverter<'a, 'b> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut args = self.args.clone();

        let mut chars = self.format.chars().peekable();

        loop {
            let Some(c) = chars.next() else {
                break;
            };

            match c {
                '%' => (),
                c => {
                    f.write_fmt(format_args!("{c}"))?;
                    continue;
                }
            }

            // '%%' in a format string means write a literal % sign
            if let Some('%') = chars.peek() {
                chars.next();
                f.write_str("%")?;
                continue;
            }

            let mut justify_left = false;
            let mut always_show_sign = false;
            let mut prepend_space = false;
            let mut alternative = false;
            let mut leading_zeroes = false;

            loop {
                match chars.peek() {
                    Some('-') => justify_left = true,
                    Some('+') => always_show_sign = true,
                    Some(' ') => prepend_space = true,
                    Some('#') => alternative = true,
                    Some('0') => leading_zeroes = true,
                    _ => break,
                }
                chars.next();
            }

            let minimum_width = { read_min_width(&mut chars, &mut args, &mut justify_left) };

            let precision = 'p: {
                let Some('.') = chars.peek() else {
                    break 'p None;
                };
                chars.next();

                read_precision(&mut chars, &mut args)
            };

            // Shut up the compiler about unused variables
            // TODO: Properly implement formatting and remove this
            let (_, _, _, _, _, _, _) = (justify_left, always_show_sign, prepend_space, alternative, leading_zeroes, minimum_width, precision);

            match chars.next() {
                None => panic!("Printf format string ended after %"),
                // Char
                Some('c') => f.write_fmt(format_args!("{}", unsafe {
                    let c: u8 = args.arg::<core::ffi::c_char>().try_into().unwrap();
                    c as char
                }))?,
                // Signed int
                Some('d' | 'i') => f.write_fmt(format_args!("{}", unsafe {
                    args.arg::<core::ffi::c_int>()
                }))?,
                // Unsigned int 
                Some('u') => f.write_fmt(format_args!("{}", unsafe {
                    args.arg::<core::ffi::c_uint>()
                }))?,
                Some('o') => {
                    let precision = precision.unwrap_or(1);

                    if alternative {
                        f.write_str("0o")?;
                    }

                    f.write_fmt(format_args!("{:0>precision$o}", unsafe {
                        args.arg::<core::ffi::c_uint>()
                    }))?;
                },
                Some('x') => f.write_fmt(format_args!("{:x}", unsafe {
                    args.arg::<core::ffi::c_uint>()
                }))?,
                Some('X') => f.write_fmt(format_args!("{:X}", unsafe {
                    args.arg::<core::ffi::c_uint>()
                }))?,


                // String
                Some('s') => f.write_fmt(format_args!("{}", unsafe {
                    CStr::from_ptr(args.arg::<*const i8>()).to_str().expect(
                        "String given as arg to printf should have been valid utf-8",
                    )
                }))?,

                // Pointer
                Some('p') => f.write_fmt(format_args!("{:p}", unsafe {
                    args.arg::<*const core::ffi::c_void>()
                }))?,

                // No hurry on implementing this because it doesn't look like it's used in ACPICA
                Some('n') => todo!("'%n' formatter which writes the current number of bytes to a pointer"),
                Some(s@ ('h' | 'l' | 'j' | 'z' | 't' | 'L')) => todo!("Format modifier '{s}'"),
                Some(s @ ('f' | 'F' | 'e' | 'E' | 'a' | 'A' | 'g' | 'G')) => panic!("Formatter '{s}' is not supported because floating point numbers are not VaArgSafe"),
                 Some(s) => panic!("Unknown printf format specifier '{s}'"),
            }
        }

        Ok(())
    }
}

#[export_name = "AcpiOsPrintf"]
unsafe extern "C" fn acpi_os_printf(format: *const i8, mut args: ...) {
    let format = unsafe { CStr::from_ptr(format) };
    let format = format
        .to_str()
        .expect("Printf string should have been valid utf-8");

    // trace!(target: "acpi_os_printf", "Format string is {format:?}");

    let mut interface = OS_INTERFACE.lock();
    let interface = interface.as_mut().unwrap();

    interface.printf(format_args!(
        "{}",
        CFmtConverter {
            format,
            args: &mut args
        }
    ));
}

#[export_name = "AcpiOsVprintf"]
unsafe extern "C" fn acpi_os_v_printf(format: *const u8, mut args: ...) {
    let format = unsafe { CStr::from_ptr(format.cast()) };
    let format = format
        .to_str()
        .expect("Printf string should have been valid utf-8");

    // trace!(target: "acpi_os_v_printf", "Format string is {format:?}");

    let mut interface = OS_INTERFACE.lock();
    let interface = interface.as_mut().unwrap();

    interface.printf(format_args!(
        "{}",
        CFmtConverter {
            format,
            args: &mut args
        }
    ));
}

#[cfg(test)]
#[allow(clippy::cast_lossless)]
mod tests {
    use core::ffi::c_int;

    use alloc::{boxed::Box, ffi::CString, fmt::format, vec::Vec};

    use crate::{handler::OsInterface, testing::DummyHandler};

    use super::*;

    macro_rules! printf_test {
        ($fn_name: ident, $result: expr, $format_str: expr, {$($name: ident: $arg: expr),*}, $($value: expr),*) => {
            #[test]
            fn $fn_name() {
                let mut handler = DummyHandler::new();
                handler.fn_printf = Box::new(|args| assert_eq!($result, format(args)));

                *OS_INTERFACE.lock() = Some(OsInterface {
                    handler: Box::new(handler),
                    objects_to_drop: Vec::new(),
                });

                let cstr = &CString::new($format_str).unwrap();

                $(
                    let $name = $arg;
                )*

                unsafe {
                    acpi_os_printf(cstr.as_ptr(), $($value),*);
                }
            }
        };
    }

    printf_test!(test_printf_const_str, "Hello", "Hello", {},);
    printf_test!(test_printf_double_percent, "Hello %", "Hello %%", {},);
    printf_test!(test_printf_s, "Hello World", "Hello %s", {s: CString::new("World").unwrap()}, s.as_ptr());
    printf_test!(test_printf_c, "Hello A", "Hello %c", {}, b'A' as c_int);
    printf_test!(test_printf_d, "Hello 100", "Hello %d", {}, 100 as c_int);
    printf_test!(test_printf_i, "Hello 100", "Hello %i", {}, 100 as c_int);
    printf_test!(
        test_printf_o,
        "Hello 0o500",
        "Hello 0o%o",
        {},
        0o500 as c_int
    );
    printf_test!(
        test_printf_o_precision,
        "Hello 0o00500",
        "Hello 0o%.5o",
        {},
        0o500 as c_int
    );
    printf_test!(
        test_printf_o_alternate,
        "Hello 0o00500",
        "Hello %#.5o",
        {},
        0o500 as c_int
    );
    printf_test!(
        test_printf_o_precision_zero_no_chars,
        "Hello ",
        "Hello %.0o",
        {},
        0o0 as c_int
    );
}
