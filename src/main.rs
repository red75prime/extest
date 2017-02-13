#[macro_use]
extern crate error_chain;
mod internal {
    error_chain! { 
        // Определяет новый тип ошибки internal::Error
        // и определяет Result<T> как std::Result<T, internal::Error>
    } 
    pub fn fallible1(v: u32) -> Result<u32> {
        if v<5000 {
            Ok(v)
        } else {
            Err("fallible1".into())
        }
    }
}

error_chain! {
    links {
        // Определяет новый тип ошибки Error
        // создает функцию преобразования из
        // internal::Error в Error
        Internal(internal::Error, internal::ErrorKind);
    }
}

fn fallible2(v: u32) -> Result<u32> {
    let v = internal::fallible1(v)?;
    if v<10000 {
        Ok(v)
    } else {
        Err("fallible2".into())
    }    
}

fn main() {
    let sum: u64 = 
        (0..100_000_000)
        .map(|v|fallible2(v%5000).unwrap_or(0) as u64)
        .sum();
    println!("Sum: {}", sum);
}
