~~kotlinx.serialization was better than serde to me~~


"You cannot use serde for serialization or deserialization if you want to serialize an enum variant index as a varint, as serde_repr cannot fulfill the requirement for varint encoding."


원래 네트워크 프로토콜은 전부다 big 엔디안인데 이러면 little 엔디안인 ARM은 패킷 디코딩 인코딩 성능을 대략 0.0000000001% 정도 손해봅니다 . 하지만 잘 들여다보면요 프로토콜에 엔디안을 정해둬서 생기는 성능 감소는 이뿐만이 아니에요. 객체에 패딩이 없다고 가정하면 그 객체를 패킷 데이타로 인코딩 디코딩할 때 단순카피만 하면되는데 엔디안이 다르니까 객체의 필드 하나하나씩 쌓아나가듯이 인코딩 디코딩해야 되요. 게다가 문제가 하나 더 있는데 단순카피 연산을 할 때는 버퍼의 남은 바이트 수를 체크하는 작업을 한번만 하지만 객체의 필드를 하나씩 인코딩 디코딩하면 남은 바이트 수를 필드의 갯수만큼 체크하게 되버리죠. 
반면에 네트워크 프로토콜에 엔디안을 강제하지 않으면 핸드쉐이크할 때 서버가 엔디안 정보를 전송하고 나서 패딩 없는 객체를 그대로 카피해서 보낼 수 있게 됩니다. 
그래서 저는 아예 매크로로 저의 게임 서버 패킷은 전부다 대략 필드 2~3개당 0.1ns 이득봅니다^^
```
sandbox    fastest       │ slowest       │ median        │ mean         
├─ decode  2.013 ns      │ 9.097 ns      │ 2.096 ns      │ 2.093 ns     
╰─ encode  1.013 ns      │ 2.389 ns      │ 1.055 ns      │ 1.198 ns     


sandbox    fastest       │ slowest       │ median        │ mean         
├─ decode  1.888 ns      │ 15.8 ns       │ 1.972 ns      │ 1.978 ns     
╰─ encode  0.847 ns      │ 1.096 ns      │ 0.93 ns       │ 0.917 ns     
```
