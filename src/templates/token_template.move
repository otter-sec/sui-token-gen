module tokengen::{{ module_name }} {
    use sui::coin;
    public struct {{ token_type }} has drop {}

    /// Initialize the token with treasury and metadata
    fun init(witness: {{ token_type }}, ctx: &mut TxContext) {
        let (treasury, metadata) = coin::create_currency(
            witness, {{ decimals }}, b"{{ symbol }}", b"{{ name }}", b"{{ description }}", option::none(), ctx
        );
        {% if is_frozen %}
        transfer::public_freeze_object(metadata);
        {% else %}
        transfer::public_transfer(metadata, ctx.sender());
        {% endif %}
        transfer::public_transfer(treasury, ctx.sender());
    }
}
