use crate::Parse;


const REGULAR: &str = r"^(get)([\s]+)([\w]+)$";
const SPLIT_REGULAR: &str = r"[\s]+";
const SIZE: usize = 2;


#[derive(Debug)]
pub struct GetParse{
    pub key: String,
}

impl Parse for GetParse {
    fn new(text: &str) -> Result<Self, ()> {
        let r = match regex::Regex::new(REGULAR) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("创建正则异常, 异常信息为:{}", e);
                return Err(())
            }
        };
        if !r.is_match(text) {
            return Err(())
        }
        let r = regex::Regex::new(SPLIT_REGULAR).unwrap();
        let result: Vec<&str> = r.splitn(text, SIZE).collect();
        let key = *&result[1];
        Ok(GetParse {
            key: key.to_string(),
        })
    }
}


#[test]
fn test() {
    let st = "get sadasfa";
    let result = GetParse::new(st).unwrap();
    println!("{:?}", result);
}