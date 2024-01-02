use std::error::Error;

pub fn from_ut8_unaligned(bytes: &[u8]) -> Result<String, Box<dyn Error>> {
    let mut result = Vec::new();
    for byte in bytes {
        let char = char::from_u32(*byte as u32).ok_or("Invalid UTF-8")?;
        result.push(char);
    }
    
    Ok(result.into_iter().collect())
}

macro_rules! periodic_for {
    ($period:expr, $item:ident in $iter:expr, $body:block) => {
        let mut next_time = std::time::Instant::now() + $period;
        for $item in $iter {
            $body
            let diff_time = next_time - std::time::Instant::now();
            if !diff_time.is_zero() {
                std::thread::sleep(diff_time);
            }
            next_time += $period;
        }
    };
}
pub(crate) use periodic_for;

macro_rules! periodic_while {
    ($period:expr, $cond:expr, $body:block) => {
        let mut next_time = std::time::Instant::now() + $period;
        while $cond {
            $body
            let diff_time = next_time - std::time::Instant::now();
            if !diff_time.is_zero() {
                std::thread::sleep(diff_time);
            }
            next_time += $period;
        }
    };
}
pub(crate) use periodic_while;

// pub fn periodic_for_f<T, I, E, F, R>(period: Duration, iter: I, mut func: F) -> Result<(), E>
// where
//     I: Iterator<Item = T>,
//     E: Error,
//     F: FnMut(T) -> R,
//     R: AsResult<ErrorType = E>,
// {
//     let mut next_time = Instant::now() + period;
//     for item in iter {
//         func(item).as_result()?;
//         let diff_time = next_time - Instant::now();
//         if !diff_time.is_zero() {
//             sleep(diff_time);
//         }
//         next_time += period;
//     }

//     Ok(())
// }

// pub fn periodic_while<E, F1, F2, R>(period: Duration, mut condition: F1, mut func: F2) -> Result<(), E>
// where
//     E: Error,
//     F1: FnMut() -> bool,
//     F2: FnMut() -> R,
//     R: AsResult<ErrorType = E>,
// {
//     let mut next_time = Instant::now() + period;
//     while condition() {
//         func().as_result()?;
//         let diff_time = next_time - Instant::now();
//         if !diff_time.is_zero() {
//             sleep(diff_time);
//         }
//         next_time += period;
//     }

//     Ok(())
// }

// The macro versions of these functions were not used becayse
// 1. I'm not sure how many imports this would cause (and the delay on each use statement)

// macro_rules! periodic_for {
//     ($period:expr, $iter:expr, $body:expr) => {
//         (|| -> Result<(), Box<dyn Error>> {
//             use $crate::as_result::AsResult;
//             let mut next_time = Instant::now() + $period;
//             for item in $iter {
//                 $body(item).as_result()?;
//                 let diff_time = next_time - Instant::now();
//                 if !diff_time.is_zero() {
//                     sleep(diff_time);
//                 }
//                 next_time += $period;
//             }
//             Ok(())
//         })()
//     };
// }
// pub(crate) use periodic_for;

// macro_rules! periodic_while {
//     ($period:expr, $cond:expr, $body:expr) => {
//         (|| -> Result<(), Box<dyn Error>> {
//             use $crate::as_result::AsResult;
//             let mut next_time = Instant::now() + $period;
//             while $cond {
//                 $body().as_result()?;
//                 let diff_time = next_time - Instant::now();
//                 if !diff_time.is_zero() {
//                     sleep(diff_time);
//                 }
//                 next_time += $period;
//             }
//             Ok(())
//         })()
//     };
// }
// pub(crate) use periodic_while;

#[cfg(test)]
mod tests {
    use super::*;

    use std::{fs, time::Duration};

    #[test]
    fn test_from_ut8_unaligned() {
        let bytes = [0x4E, 0x65, 0x75, 0x72, 0x6F, 0x6E];
        let string = from_ut8_unaligned(&bytes).unwrap();
        assert_eq!(string, "Neuron");
    }

    #[test]
    fn test_periodic_for() -> Result<(), Box<dyn Error>> {
        let period = Duration::from_millis(10);
        periodic_for!(period, item in 0..10, {
            let _ = item + 1;
        });

        Ok(())
    }

    #[test]
    fn test_periodic_for_error() -> Result<(), Box<dyn Error>> {
        let period = Duration::from_millis(10);
        assert!(
            || -> Result<(), Box<dyn Error>> {
                periodic_for!(period, item in 0..10, {
                    fs::File::open("nonexistent_file.txt")?;
                    let _ = item + 1;
                });
                Ok(())
            }().is_err()
        );

        Ok(())
    }

    #[test]
    fn test_periodic_while() -> Result<(), Box<dyn Error>> {
        let period = Duration::from_millis(10);
        let mut i = 0;
        periodic_while!(period, i < 10, {
            i += 1;
        });

        Ok(())
    }

    #[test]
    fn test_periodic_while_error() -> Result<(), Box<dyn Error>> {
        let period = Duration::from_millis(10);
        assert!(
            || -> Result<(), Box<dyn Error>> {
                let mut i = 0;
                periodic_while!(period, i < 10, {
                    fs::File::open("nonexistent_file.txt")?;
                    i += 1;
                });
                Ok(())
            }().is_err()
        );

        Ok(())
    }
}