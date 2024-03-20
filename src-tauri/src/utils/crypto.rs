mod crypto {
    //! The `crypto` module contains functions about crypto.
    //! 
    //! # Install Dependencies
    //! 
    //! ```
    //! $ cargo add base64 sha2 url
    //! ```
    
    /// Encodes str to base64 string.
    /// Takes care if the input data is url, see [URL-safe alphabet](https://docs.rs/base64/latest/base64/#url-safe-alphabet).
    /// 
    /// # Arguments
    /// 
    /// * `data` - A string that holds the text data
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crypto::encoding_base64;
    /// 
    /// assert_eq!(encode_base64("Hello world!").expect("Error!"), "SGVsbG8gd29ybGQh");
    /// ```
    pub fn encode_base64(data: &str) -> Result<String, ()> {
        use base64::{engine::general_purpose::STANDARD, engine::general_purpose::URL_SAFE,Engine as _};
        use url::Url;

        if let Err(_error) = Url::parse(data) {
            return Ok(STANDARD.encode(data));
        } else {
            return Ok(URL_SAFE.encode(data));
        }
    }
    
    /// Decodes base64 data to normal string.
    /// Takes care if the origin data is url, see [URL-safe alphabet](https://docs.rs/base64/latest/base64/#url-safe-alphabet).
    /// 
    /// # Arguments
    /// 
    /// * `data` - A string that holds the encoded data.
    /// * `is_url` - Option if the origin data is url.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crypto::decode_base64;
    /// 
    /// assert_eq!(decode_base64("SGVsbG8gd29ybGQh", None).expect("Error!"), "Hello world!");
    /// ```
    pub fn decode_base64(data: &str, is_url: Option<bool>) -> Result<String, base64::DecodeError> {
        use base64::{engine::general_purpose::STANDARD, engine::general_purpose::URL_SAFE,Engine as _, DecodeError};
    
        if let Some(true) = is_url {
            match URL_SAFE.decode(data) {
                Ok(arr) => {
                    if let Ok(res) = String::from_utf8(arr) {
                        return Ok(res);
                    } else {
                        return Err(DecodeError::from(DecodeError::InvalidPadding));
                    }
                },
                Err(error) => {
                    // println!("decode error, errMsg: {:?}", error);
                    return Err(error);
                }
            }           
        } else {
            match STANDARD.decode(data) {
                Ok(arr) => {
                    if let Ok(res) = String::from_utf8(arr) {
                        return Ok(res);
                    } else {
                        return Err(DecodeError::from(DecodeError::InvalidPadding));
                    }
                },
                Err(error) => {
                    // println!("decode error, errMsg: {:?}", error);
                    return Err(error);
                }
            }            
        }
    }

    /// Encodes file to Sha256 string.
    /// 
    /// # Arguments
    /// 
    /// * `path` - A string that holds the file path.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crypto::encode_sha2_file;
    /// 
    /// let path = "path\\to\\encoded";
    /// 
    /// match encode_sha2_file(path) {
    ///     Ok(data) => {
    ///         println!("File {} encoded data: {}", path, data);
    ///     },
    ///     Err(error) => {
    ///         println!("Failed to encode file {}, errMsg: {:?}", path, error);
    ///     }
    /// }
    /// ```
    pub fn encode_sha2_file(path: &str) -> Result<String, std::io::Error> {
        use sha2::{Sha256, Digest};
        use std::io::{Error, ErrorKind};
        use std::{fs, path::Path};
        
        let input = Path::new(path);
        if !input.exists() {
            return Err(Error::from(ErrorKind::NotFound));
        }
    
        let mut hasher = Sha256::new();
    
        match fs::read(path) {
            Ok(data) => {
                hasher.update(data);
                return Ok(format!("{:X}", hasher.finalize()));
            },
            Err(_error) => {
                // println!("encode sha2 file for {} failed, errMsg: {:?}", path, error);
                return Err(Error::from(ErrorKind::Other));
            }
        }
    }
}
