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

예를 들어, Bitcoin 네트워크에서 Hard Fork 의 결과 Bitcoin Cash 가 추가 생성됨. Bitcoin Cash는 Bitcoin의 규칙을 변경하여 Block 크기를 키우는 것을 목표로 하였고, 이를 위해 Hard Fork 를 진행함. 이로 인해 Bitcoin과 Bitcoin Cash 두 가지 다른 Blockchain이 생겨나게 됨.


## 3. Bitcoin Improvement Proposal (BIP) 9
Bitcoin Improvement Proposal (BIP) 9는 Bitcoin 네트워크에서 Soft For를 배포하는 메커니즘.<br> 
이 BIP는 이전 BIP 버전에 비해 Soft Fork 활성화 과정을 더 유연하고 예측 가능하게 만듬.<br><br>

BIP 9는 버전 비트를 이용해 새로운 Soft Fork 규칙의 도입을 제어함. 이 bit는 Bitcoin 네트워크의 miner들이 새로운 규칙을 지원하는지를 나타내는 역할을 함. 이 bit가 켜지면 새로운 규칙이 활성화되고, bit가 꺼지면 해당 규칙이 비활성화됨.<br><br>

BIP 9의 작동 방식

* 정의 (Defined): BIP 9는 Soft Fork를 정의하고 그에 대한 세부 사항을 제공.<br>

* 시작 (Started): 이 단계에서 miner들은 새로운 규칙을 준수하는 Block을 생성할 수 있음.<br>

* 잠김 (Locked In): 네트워크의 95% 이상이 새로운 규칙을 지원하는 Block을 생성하면 해당 규칙이 잠김. 이 단계를 통과하면 새로운 규칙이 활성화될 것이라는 것이 확정됨.<br>

* 활성화 (Activated): 새로운 규칙이 잠긴 후 일정 시간이 지나면 해당 규칙이 활성화. 이제 모든 Block은 새로운 규칙을 준수해야.<br><br>

실패 (Failed): 만약 네트워크의 95% 이상이 새로운 규칙을 지원하지 않는 Block을 생성하는 경우, Soft Fork는 실패한 것으로 간주되며, 이때는 새로운 규칙이 활성화되지 않음.<br><br>

BIP 9의 목표는 네트워크 참여자들이 Soft Fork의 상태를 쉽게 파악하고, 미래의 변화에 대비할 수 있게 하는 것으로,Bitcoin 네트워크의 upgrade 를 더욱 안전하고 원활하게 만들기 위해 사용.<br>


## 4. Bitcoin Improvement Proposal 141 (BIP 141), 또는 Segregated Witness (SegWit)

Bitcoin Improvement Proposal 141 (BIP 141), 또는 Segregated Witness (SegWit)는 Bitcoin 네트워크의 Transaction 용량을 향상시키기 위한 주요 update를 제안.<br><br>

SegWit는 Block 내의 Transaction 구조를 변경하여 이전에 발생했던 특정한 문제들을 해결하려는 것으로, 그 중 하나는 "Transaction 용량" 문제였음. Bitcoin Block의 크기는 1MB로 제한되어 있었는데, 이는 초당 처리할 수 있는 Transaction 수를 제한하는 결과를 가져오게 됨.<br><br>

SegWit는 이 문제를 해결하기 위해 "witness"라는 Transaction 데이터를 별도로 분리해 보관함으로써 Block의 용량을 더 효율적으로 활용가능 해짐. 이렇게 하면 Transaction의 양이 증가하면서도 Block의 크기는 동일하게 유지할 수 있게 되어, Transaction 처리량을 증가시킬 수 있게 되었음.<br><br>

또한, SegWit는 Transaction 재정렬(malleability) 문제를 해결함. 이 문제는 Transaction의 고유 식별자를 변경하면서 이를 악용할 수 있는 취약점이었는데, SegWit는 이를 해결하여 더욱 안전한 네트워크를 구축할 수 있게 도움을 줌.<br><br>

BIP 141은 처음에는 95%의 miner들의 합의가 필요했으나, BIP 91이 적용된 이후에는 80%의 합의만으로도 SegWit의 적용이 가능하게 되어, SegWit는 2017년에 성공적으로 활성화됨.<br><br>


## 5. Bitcoin Improvement Proposal (BIP) 91

Bitcoin Improvement Proposal 91 (BIP 91)은 Bitcoin 네트워크의 Soft Frok 배포 방식을 개선하기 위한 제안임.<br><br>

BIP 91의 주요 목표는 Segregated Witness (SegWit)라는 update를 더 쉽게 배포하고 활성화하는 것으로 SegWit는 Bitcoin Transaction의 크기를 줄이고 네트워크의 확장성을 향상시키기 위한 주요 update 임.<br><br>

SegWit의 배포는 원래 BIP 141을 통해 이루어지려 했지만, BIP 141은 네트워크의 95% 이상이라는 높은 비율의 hash power를 가진 miner들이 이를 지원해야 활성화되는 조건이 SegWit의 배포를 어렵게 만듬.<br><br>

이 문제를 해결하기 위해 제안된 것이 BIP 91 으로, BIP 91은 SegWit의 활성화를 위한 miner들의 합의 비율을 95%에서 80%로 낮춤. 이는 SegWit의 배포를 쉽게 만들고, Bitcoin 네트워크의 확장성 향상을 위한 중요한 사항 중 하나.<br><br>

BIP 91은 2017년 7월에 성공적으로 활성화되었고, 이후 SegWit update가 Bitcoin 네트워크에 성공적으로 배포됨. 이로써 Bitcoin 네트워크는 더 많은 Transaction을 처리할 수 있게 되었음.<br><br>


