pub mod ixs;

/// AMM定价：给定虚拟base、虚拟quote和买入quote数量，返回可获得的base数量（不含手续费）
/// x: virtual_base, y: virtual_quote, dy: quote_in
/// 公式：base_out = x - (x * y) / (y + dy)
pub fn get_out_amount(virtual_base: u128, virtual_quote: u128, quote_in: u128) -> u128 {
    if quote_in == 0 || virtual_base == 0 || virtual_quote == 0 {
        return 0;
    }
    let y_new = virtual_quote + quote_in;
    let xy = virtual_base.saturating_mul(virtual_quote);
    let x_new = xy / y_new;
    let base_out = virtual_base.saturating_sub(x_new);
    base_out
}
