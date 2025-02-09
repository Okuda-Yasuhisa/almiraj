pub mod bitset;

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveType {
    // 2値型（booleanなど）
    Bit,     // 1-bit (0 or 1) - bool型の代替
    // 量子化用の小さなビットサイズ
    UInt2,   // 2-bit (0 to 3)
    UInt4,   // 4-bit (0 to 15)
    // 既存の型
    UInt8,   // 0 to 255
    UInt16,  // 0 to 65,535
    UInt32,  // 0 to 4,294,967,295
    UInt64,  // 0 to 18,446,744,073,709,551,615
    Int8,    // -128 to 127
    Int16,   // -32,768 to 32,767
    Int32,   // -2,147,483,648 to 2,147,483,647
    Int64,   // -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
}

impl PrimitiveType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "bit" | "bool" => Some(PrimitiveType::Bit),
            "uint2" => Some(PrimitiveType::UInt2),
            "uint4" => Some(PrimitiveType::UInt4),
            // 既存の型
            "uint8" => Some(PrimitiveType::UInt8),
            "uint16" => Some(PrimitiveType::UInt16),
            "uint32" => Some(PrimitiveType::UInt32),
            "uint64" => Some(PrimitiveType::UInt64),
            "int8" => Some(PrimitiveType::Int8),
            "int16" => Some(PrimitiveType::Int16),
            "int32" => Some(PrimitiveType::Int32),
            "int64" => Some(PrimitiveType::Int64),
            _ => None,
        }
    }

    pub fn size_in_bits(&self) -> u8 {
        match self {
            PrimitiveType::Bit => 1,
            PrimitiveType::UInt2 => 2,
            PrimitiveType::UInt4 => 4,
            PrimitiveType::UInt8 | PrimitiveType::Int8 => 8,
            PrimitiveType::UInt16 | PrimitiveType::Int16 => 16,
            PrimitiveType::UInt32 | PrimitiveType::Int32 => 32,
            PrimitiveType::UInt64 | PrimitiveType::Int64 => 64,
        }
    }

    // メモリ上での実際のバイトサイズを返す（1バイト未満は1バイトにアライメント）
    pub fn size_in_bytes(&self) -> u8 {
        (self.size_in_bits() + 7) / 8
    }

    // 実際のメモリ使用量を計算
    pub fn memory_alignment(&self) -> u8 {
        self.size_in_bits()
    }

    // 値の範囲を返す
    pub fn value_range(&self) -> (i64, i64) {
        match self {
            PrimitiveType::Bit => (0, 1),
            PrimitiveType::UInt2 => (0, 3),
            PrimitiveType::UInt4 => (0, 15),
            PrimitiveType::UInt8 => (0, 255),
            PrimitiveType::UInt16 => (0, 65_535),
            PrimitiveType::UInt32 => (0, 4_294_967_295),
            PrimitiveType::UInt64 => (0, 18_446_744_073_709_551_615),
            PrimitiveType::Int8 => (-128, 127),
            PrimitiveType::Int16 => (-32_768, 32_767),
            PrimitiveType::Int32 => (-2_147_483_648, 2_147_483_647),
            PrimitiveType::Int64 => (-9_223_372_036_854_775_808, 9_223_372_036_854_775_807),
        }
    }

    /// ビット変数をグループ化してBitSet8を作成するかを判断
    pub fn should_use_bitset(&self) -> bool {
        matches!(self, PrimitiveType::Bit)
    }
}
