// Almirajの関数定義を表現する構造体Function

pub struct Function {
    pub is_global: bool,
    pub is_public: bool,
    pub return_type: String,
    pub name: String,
    // 各引数は (型, 引数名) のペアで管理
    pub arguments: Vec<(String, String)>,
    // 関数本体の各行を文字列として保持
    pub body: Vec<String>,
}

impl Function {
    // 新規関数定義の作成。引数は (型, 名前) のVecを指定
    pub fn new(is_global: bool, return_type: &str, name: &str, arguments: Vec<(String, String)>) -> Self {
        Self {
            is_global,
            is_public: false,
            return_type: return_type.to_string(),
            name: name.to_string(),
            arguments,
            body: Vec::new(),
        }
    }

    // 関数本体に1行追加するためのヘルパー
    pub fn add_line_to_body(&mut self, line: &str) {
        self.body.push(line.to_string());
    }
}