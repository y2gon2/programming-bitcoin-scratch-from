## 1. Soft Fork

Blockchain update 방식 중 하나. <br><br>
기존 Blockchain 규칠르 수정하거나 새로운 규칙을 추가하는 방식으로, 기존의 규칙과 호환되는 방식 기존 node 가 update 를 받지 않더라고, 새로운 규칙에 따라 만들어진 block 은 여전히 유효하다고 인식할 수 있음.
하지만 반대로 update 를 받지 않은 node 가 생성하는 block 은 새로운 규칙을 받은 node 들에게는 유효하지 않을 수 있음.<br><br>

이러한 방식은 Blockchain 의 변경은 부드럽게 진행 할 수 있게 해주며, 모든 node 가 동시에 update 를 진행하지 않아도 네트워크가 계속 작동하며, node 들은 필요에 따라서 update 를 진행할 수 있음 <br><br>

Soft Fork 는 특히 주요한 변경을 네트워크에 안전하게 도입할때 사용되며, 이는 네트워크 참가자들 사이의 합의를 통해 이루어짐. bitcoin 에서의 주요 soft fork 예시로는 Segregated Witness (SegWit) 가 있음.<br>


## 2. Hard Fork

Soft Fork 와 반대로 기존 Blockchain 규칙과 호환되지 않는 update 를 의미<br>

Hard Fork 는 새로운 규칙에 따라 생성된 block이 기존 규칙을 따르는 node 에게는 유효하지 않게 만듬. 즉, Hard Fork 가 발생하면 네트워크가 두 가지 버전으로 나뉘게 되며, 하나는 updae 이전의 규칙을 따르고, 다른 하나는 새로운 규칙을 따르게 됨.<br>

이는 기존 Blockchain 과 새로운 Blockchain 사이에 호환성이 없게 되어, 모든 네트워크 참가자들이 새로운 규칙을 수용하고 update 를 해야만 네트워크가 정상적으로 작동하게 됨. 이런 이유로 Hard Fork는 주로 큰 변화나 개선을 도입할 때 사용되며, 이런 경우에는 보통 새로운 암호화폐가 생기는 결과를 초래.<br><br>

예를 들어, Bitcoin 네트워크에서 Hard Fork 의 결과로 비트코인 캐시(Bitcoin Cash)가 생성함. Bitcoin Cash는 Bitcoin의 규칙을 변경하여 Block 크기를 키우는 것을 목표로 하였고, 이를 위해 Hard Fork 를 진행함. 이로 인해 Bitcoin과 Bitcoin Cash 두 가지 다른 Blockchain이 생겨나게 됨.

## 3. Bitcoin Improvement Proposal (BIP) 9
Bitcoin Improvement Proposal (BIP) 9는 Bitcoin 네트워크에서 Soft For를 배포하는 메커니즘.<br> 
이 BIP는 이전 BIP 버전에 비해 Soft Fork 활성화 과정을 더 유연하고 예측 가능하게 만듬.<br><br>

BIP 9는 버전 비트를 이용해 새로운 Soft Fork 규칙의 도입을 제어함. 이 bit는 Bitcoin 네트워크의 마이너들이 새로운 규칙을 지원하는지를 나타내는 역할을 함. 이 bit가 켜지면 새로운 규칙이 활성화되고, bit가 꺼지면 해당 규칙이 비활성화됨.<br><br>

BIP 9의 작동 방식:<br><br>

* 정의 (Defined): BIP 9는 Soft Fork를 정의하고 그에 대한 세부 사항을 제공.<br>

* 시작 (Started): 이 단계에서 마이너들은 새로운 규칙을 준수하는 블록을 생성할 수 있음.<br>

* 잠김 (Locked In): 네트워크의 95% 이상이 새로운 규칙을 지원하는 블록을 생성하면 해당 규칙이 잠김. 이 단계를 통과하면 새로운 규칙이 활성화될 것이라는 것이 확정됨.<br>

* 활성화 (Activated): 새로운 규칙이 잠긴 후 일정 시간이 지나면 해당 규칙이 활성화. 이제 모든 블록은 새로운 규칙을 준수해야.<br><br>

실패 (Failed): 만약 네트워크의 95% 이상이 새로운 규칙을 지원하지 않는 블록을 생성하는 경우, Soft Fork는 실패한 것으로 간주되며, 이때는 새로운 규칙이 활성화되지 않음.<br><br>

BIP 9의 목표는 네트워크 참여자들이 Soft Fork의 상태를 쉽게 파악하고, 미래의 변화에 대비할 수 있게 하는 것으로,Bitcoin 네트워크의 upgrade 를 더욱 안전하고 원활하게 만들기 위해 사용.<br><br>