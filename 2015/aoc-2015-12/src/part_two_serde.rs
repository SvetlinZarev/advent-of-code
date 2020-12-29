use serde_json::value::Value;

const KEY_RED: &'static str = "red";

pub fn solve(input: &str) -> i64 {
    let parsed: Value = serde_json::from_str(input).unwrap();

    let data = parsed.as_array().unwrap();
    handle_iterator(data)
}

fn handle_iterator<'l, I>(array: I) -> i64
where
    I: IntoIterator<Item = &'l Value>,
{
    let mut sum = 0;

    for obj in array {
        match obj {
            Value::Number(x) => sum += x.as_i64().unwrap(),
            Value::Array(array) => sum += handle_iterator(array),
            Value::Object(obj) => {
                // process the object's values as early as possible in order to
                // avoid unnecessary recursion and iteration
                let has_red = obj
                    .values()
                    .filter(|&x| x.is_string())
                    .find(|&x| x.as_str().unwrap().eq(KEY_RED))
                    .is_some();

                if !has_red {
                    sum += handle_iterator(obj.values())
                }
            }
            _ => { /* ignore */ }
        }
    }

    sum
}
