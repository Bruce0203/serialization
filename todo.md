# 높은 우선순위 

1. vec len
2. decode
3. enum 

var-int 방법과 와 websocket packet length 방법의 속도 차이 알아보기
enum 객체를 만들고  Walker에서 enum을 다루도록 하기 그러기 위해서 sort와 flatten과 len과 pad에 Enum핸들링 구현블럭을 추가하기  
Vec의 크기 계산을 위한 VarInt만들기 
min size 또는 max size가 정해진 경우 고려하기

~~enum에 edge를 달기 위해서 Enum::Variant(MaybeUninit)을 하는 함수 만들어서 필드 오프셋 구하고~~
enum variant index 구하는 함수 만들고
decode에 populate 도 추가해야 함 

---

#  중간 수준의 우선순위

remove generic_const_exprs from lib.rs of serialization crate => pad.rs 수정하기 
Model만 repr(Rust)의 영향을 받는다 Bar, Foo도 repr(Rust)하면 repr(C)와 달라지게 테스트하라
mock 구현과 실제 구현 합일화하기

---

# 낮은 우선순위

vectored<T, V>를 Vectored<T>로 바꾸기 
vec.rs를 vec/으로 바꾸기 
rename Len::SIZE to Len::LEN
[취소됨;위험함] 대부분의 S, S2 구분 없애기 -> Flatten 트레잇에 <S> 제네릭 붙여서 없애도 됨  단 impl Add<Rhs> for Padding  은 제외함 [취소됨;위험함]
impl macro_rules들 전부다 __붙이기  
AllPrimitiveSizeCheck 에 따라서 Edge구현하기...? -> Actor의 Field<A>로 구하기 -> 이건 Mesh<C>를 쓰지 않음으로써 해결 가능할 것 같다 단지 sort만 안하고 flatten 을 포함한 나머지 연산을 하는 것으로 해결이 가능하다 하지만 이러면 padding 연산이 어떻게 될지 모르겠는게 문제인데 Walker에 skip_len을 비활성화하는 옵션을 만들어야 겠다
learn usecase of repr(transparent)

rename pad.rs to padding.rs
decode_element의 place와 out이 혼용됨
&'static str 은 인코딩만 되서 Serializable을 Serialize, Deserialize로 분리해서 &'static str을 지원해야 함
