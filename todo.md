# 낮은 우선순위

~~string utf8 decoding validation~~
rename Len::SIZE to Len::LEN
Codec::endian()이걸로 Walker에서 cluster할지 말지 결정해 
decode_element의 place와 out이 혼용됨
__VariantToken2 를 __TypeErasedVariantToken 으로 개명하기

---

# 할 수도 있을 것 같은 것들 
enum의 simple한지 체크하고 만약 그렇다면 Enum의 Len을 UNSIZED에서 size_of::<T>()로 바꾸기 
&'a str을 Edge화 하기

---

# 하지 않을 것듣

S,S2,S3를 S로 통일하지 않을 거지만 만약 하게 된다면 impl Add<Rhs> for Padding/Vectored/Enum/..etc 은 제외함
min size 또는 max size가 정해진 경우 고려하기
mock 구현과 실제 구현 합일화하기 ->  어떻게든 delegate를 하든지 해서 encoder의 구현 중복 없애자.  ㅠㅠ
src/codec/traits.rs 를 src/로 옮기기
avro-rs 밴치마크에 추가하기 
