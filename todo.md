# 높은 우선순위 

todo: enum의 variant의 필드들 edge  체인의 시작  부분에 discriminant를 위한 ConstPadding 삽입하기
enum 지원하기 
decode에 populate 도 추가해야 함 
&'static str 은 인코딩만 되서 Serializable을 Serialize, Deserialize로 분리해서 &'static str을 지원해야 함

enum variant index 구하는 함수 만들고 -> 취소됨

---

#  중간 수준의 우선순위

export하는 impl macro_rules들 전부다 "\__"붙이기  
remove generic_const_exprs from lib.rs of serialization crate => pad.rs 수정하기 
Model만 repr(Rust)의 영향을 받는다 Bar, Foo도 repr(Rust)하면 repr(C)와 달라지게 테스트하라

---

# 낮은 우선순위

rename Len::SIZE to Len::LEN
S,S2,S3를 S로 통일하지 않을 거지만 만약 하게 된다면 impl Add<Rhs> for Padding/Vectored/Enum/..etc 은 제외함
Codec::endian()이걸로  Walker에서 cluster할지 말지 결정해 
decode_element의 place와 out이 혼용됨
holder trait의 이름을 FooKind로 개명하기를 고려하기 
min size 또는 max size가 정해진 경우 고려하기
mock 구현과 실제 구현 합일화하기 ->  어떻게든 delegate를 하든지 해서 encoder의 구현 중복 없애자.  ㅠㅠ
src/codec/traits.rs 를 src/로 옮기기
Serializable이 아니라 Serialize가 더 낫다 왜냐하면 형용사는 수식하는 대상이 객체일 텐데 러스트에는 마땅한 객체라고 할게 자바보다 없기 때문이다
avro-rs 밴치마크에 추가하기 
__VariantToken2 를 __TypeErasedVariantToken 으로 개명하기
