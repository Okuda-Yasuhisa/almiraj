use std::collections::HashMap;

/// 8つのboolean値を1バイトにパックする構造体
pub struct BitSet8 {
    /// 実際のビット値を保持するバイト
    bits: u8,
    /// ビット位置と変数名のマッピング
    bit_map: HashMap<String, usize>, // (変数名, ビット位置)
    /// 次に割り当て可能なビット位置
    next_bit: usize,
}

impl BitSet8 {
    pub fn new() -> Self {
        Self {
            bits: 0,
            bit_map: HashMap::new(),
            next_bit: 0,
        }
    }

    /// 新しいビット変数を追加
    pub fn allocate_bit(&mut self, var_name: &str) -> Option<usize> {
        if self.next_bit >= 8 {
            return None; // ビットが満杯
        }
        let bit_pos = self.next_bit;
        self.bit_map.insert(var_name.to_string(), bit_pos);
        self.next_bit += 1;
        Some(bit_pos)
    }

    /// 変数の値を設定
    pub fn set(&mut self, var_name: &str, value: bool) -> bool {
        if let Some(bit_pos) = self.bit_map.get(var_name) {
            if value {
                self.bits |= 1 << bit_pos;
            } else {
                self.bits &= !(1 << bit_pos);
            }
            true
        } else {
            false
        }
    }

    /// 変数の値を取得
    pub fn get(&self, var_name: &str) -> Option<bool> {
        self.bit_map.get(var_name).map(|bit_pos| {
            (self.bits & (1 << bit_pos)) != 0
        })
    }

    /// 使用中のビット数を返す
    pub fn used_bits(&self) -> usize {
        self.next_bit
    }

    /// 利用可能なビット数を返す
    pub fn available_bits(&self) -> usize {
        8 - self.next_bit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitset8() {
        let mut bits = BitSet8::new();
        
        // 変数の割り当て
        assert!(bits.allocate_bit("flag1").is_some());
        assert!(bits.allocate_bit("flag2").is_some());
        
        // 値の設定と取得
        assert!(bits.set("flag1", true));
        assert!(bits.set("flag2", false));
        assert_eq!(bits.get("flag1"), Some(true));
        assert_eq!(bits.get("flag2"), Some(false));
        
        // 存在しない変数へのアクセス
        assert_eq!(bits.get("nonexistent"), None);
    }
}
