~~kotlinx.serialization was better than serde to me~~


"You cannot use serde for serialization or deserialization if you want to serialize an enum variant index as a varint, as serde_repr cannot fulfill the requirement for varint encoding."


//TODO CalcSizeState 를 Builder마냥 함수 하나에 때려박은 거 다 State구조체 만들어서 작은 함수로 나눠야 constexpr 안에 들어간다
