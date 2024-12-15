module test_token::test {
    use sui::coin;
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};

    struct TEST has drop {}

    fun init(witness: TEST, ctx: &mut TxContext) {
        let (treasury_cap, metadata) = coin::create_currency(
            witness,
            6,
            b"TEST",
            b"Test Token",
            b"A test token",
            option::none(),
            ctx
        );
        transfer::public_transfer(treasury_cap, tx_context::sender(ctx));
        transfer::public_share_object(metadata);
    }
}
