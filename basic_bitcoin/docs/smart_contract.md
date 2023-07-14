## Smart Contract & Script (Bitcoin 기준으로 작성)

### 1. Smart Contract
- Bitcoin 을 구성하는 Blockchain 에서 자동으로 실행되는 계약
- 디지털 자산간의 (기본 규약에 근거한) 거래를 자동화 하며, 특정 조건이 충족될때만 실행
- Smart Contract 들 중 자주 사용된는 조건들은 다음과 같으면, 해당 조건들이 단독으로 
  또는 복합적으로 사용될 수 있음.
  <br><br>
    * multisig (다중 서명)<br>
    다중 서명 거래는 두 명 이상의 사용자가 서명해야만 비트코인을 송금할 수 있는 방식
    예를 들어, 3명의 사용자가 있고 이들 중 2명의 서명이 필요한 경우 "2-of-3" 다중 서명 거래라고 함.
    이는 공동으로 자금을 관리하거나, 자금의 안전을 높이는 경우에 유용하게 사용
    <br><br>
    * time-locked (시간 기반 잠금) transaction<br>
    특정 시간이나 블록 수가 지나야만 비트코인을 송금할 수 있는 방식
    이를 통해 미래의 특정 시점에 자동으로 비트코인을 전송하거나, 당사자 간의 합의 없이 일정 시간이 지나면 자금을 반환하는 등의 기능을 구현 가능
    <br><br>
    * hash-locked (해시 기반 잠금) transaction<br>
    특정 해시값의 원본을 제공해야만 비트코인을 송금할 수 있는 방식
    이는 주로 "Hashed Timelock Contracts (HTLCs)"라는 형태로 사용되며, 라이트닝 네트워크와 같은 비트코인의 레이어 2 솔루션에서 중요한 역할
    HTLCs는 시간 기반 잠금과 해시로 잠긴 거래를 결합하여, 빠른 속도의 마이크로 페이먼트 채널을 가능하게 함.
    <br>
<br>

### 2. Script 란 무엇인가?
- Bitcoin Smart Contract 의 구현 방법을 제공
- Smart Contract 이 실제로 "어떻게" 실행될 것인지를 정의하는 프로그래밍 언어로서 작동
- 가 특정 조건을 충족시키는 경우에만 유효하게 되도록 하는 방식으로, Bitcoin 네트워크에서 거래의 유효성을 검증하는 데 사용
- Script 를 통해 "계약"으로 사용되는 것을 구현할 수 있는 stack 기반 언어이지만 blockchain system 및 network 에 무리를 주거나 무한한 작업을 부여할 수 있는 구현을 막고자 Turing incompleteness 의 속성을 지니도록 설계 되었다. 
  * (일반적 범용 언어는 Turing completeness 의 속성을 지님)
  * (이더리움의 Smart Contract Language 인 Solidity 의 경우 Turing completeness 속성을 지니지만 'gas' 라는 무언가 프로그램 실행의 대가로 지불해야하는 것이 존재하여 무한 루프와 같은 문제가 되는 작업의 요청 진행을 방지함.)<br><br>

### 3. Script 실행
- script 에서 주어진 명령어(command)를 한번에 하나씩 stack 을 기반으로 처리
- command 는 element 와 operator 로 이루어짐

#### element
- script 에서 사용되는 data 를 의미 (DER singniture, SEC public key)
- 1 ~ 520 bytes
- 각가의 처리할 element 를 skack 에 저장하여 사용

* Distinguished Encoding Rules (DER) <br>
  ASN.1 표준에 따라 데이터를 인코딩하는 방법을 나타냄. Bitcoin에서는 이 DER 형식을 사용하여 ECDSA(Elliptic Curve Digital Signature Algorithm) 서명을 인코딩함. DER 인코딩은 서명의 두 구성 요소인 r과 s를 표현하는 데 사용되며, 각각은 정수로 인코딩되어 있음.<br>
* Standards for Efficient Cryptography (SEC)<br>
  공개키를 인코딩하기 위한 방법을 나타냅니다. Bitcoin에서는 이 SEC 형식을 사용하여 Elliptic Curve 공개키를 인코딩합니다. SEC 인코딩은 공개키의 x와 y 좌표를 나타내며, 이는 공개키의 길이를 줄이기 위해 압축된 형식으로 사용될 수도 있습니다 <br><br>

#### operator
- element 를 가지고 목적에 필요한 수학적, 논리적 작업을 진행 
- 주요 operator
  * OP_DUP  : 해당 element 를 복사하여 stack 위에 저장
  * OP_HASH160 : 해당 element 의 hash160 로 hashing 된 값을 stack 위에 저장
  * OP_CHECKSIG: pubkey 로 signiture 가 검정되는지를 확인
  

