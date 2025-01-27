# 높은 우선순위 

enum variant index 구하는 함수 만들고
~~var-int 방법과 와 websocket packet length 방법의 속도 차이 알아보기~~ -> 그냥 usize는 u64대로 걍 카피하는 게 나음 ㅋㅋ
~~enum 객체를 만들고  Walker에서 enum을 다루도록 하기 그러기 위해서 sort와 flatten과 len과 pad에 Enum핸들링 구현블럭을 추가하기  ~~
~~Vec의 크기 계산을 위한 VarInt(?)만들기 ~~
min size 또는 max size가 정해진 경우 고려하기

~~enum에 edge를 달기 위해서 Enum::Variant(MaybeUninit)을 하는 함수 만들어서 필드 오프셋 구하고~~
decode에 populate 도 추가해야 함 

---

#  중간 수준의 우선순위

remove generic_const_exprs from lib.rs of serialization crate => pad.rs 수정하기 
Model만 repr(Rust)의 영향을 받는다 Bar, Foo도 repr(Rust)하면 repr(C)와 달라지게 테스트하라
mock 구현과 실제 구현 합일화하기

---

# 낮은 우선순위

rename Len::SIZE to Len::LEN
S,S2,S3를 S로 통일하지 않을 거지만 만약 하게 된다면 impl Add<Rhs> for Padding/Vectored/Enum/..etc 은 제외함
export하는 impl macro_rules들 전부다 __붙이기  
Codec::endian()이걸로  Walker에서 cluster할지 말지 결정해 
decode_element의 place와 out이 혼용됨
&'static str 은 인코딩만 되서 Serializable을 Serialize, Deserialize로 분리해서 &'static str을 지원해야 함
