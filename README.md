# substrate-advance-lession
substrate编程进阶课程

1.第一课作业
    1.1：编写存证模块的单元测试代码，包括：
        *创建存证的测试用例
        *撤销存证的测试用例
        *转移存证的测试用例
    1.2：创建存证时，为存证内容的哈希值Vec<u8>
        *设置长度上限，超过限制时返回错误
        *并编写测试用例
// 作业验证方式：
cd lesson1/substrate-node-template
cargo test --package pallet-poe

2.第二课作业
    2.1：增加买和卖的extrinsic,对视频中kittes的实现进行重构，提取出公共代码
    2.2：KittyIndex不在pallet中指定，而是在runtime里面绑定
    2.3：测试代码能测试所有的五个方法，能检查所有定义的event，能测试出所有定义的错误类型
    2.4：引入Balances里面的方法，在创建时质押一定数量的token，在购买时支付token
// 作业验证方式：
cd lesson2/substrate-node-template
cargo test --package pallet-kitties

3.第三课作业
    3.1：这个作业你需要用这个目录下的kitties/substrate作为Substrate端，然后以kitties/frontend 作为前端继续开发
    3.2：这个作业目的是在已有的kitties前端基础上，写出查询区块链的逻辑来显示每只猫咪
    3.3：每只猫咪需要显示其：喵咪的ID，猫咪的DNA，猫咪所属的AccountId
// 作业验证方式：
(a)区块链启动
cd lesson3/kitties/node
cargo build --release
./target/debug/node-template --dev --tmp
(b)前端启动
cd frontend
yarn insyall
yarn start

4.第四课作业
    4.1：以owc-example为基础，把它拷贝到assignment目录里来修改，最后提交这个代码库
    4.2：利用offchain worker取出DOT当前对USD的价格，并把它写到一个Vec的存储里，自己选一种方案提交回链上，并在代码注释为什么用这种方法提交回链上最好。最保留当前最近的10个价格，其他价格可以丢弃（就是Vec的长度长到10后，这时再插入一个值时，要先丢弃最早的那个值）。
    4.3：这个http请求可得到当前DOT价格
// 作业验证方式：
cd lesson3/ocw-example
cargo build --release
./target/release/node-template --dev --tmp
