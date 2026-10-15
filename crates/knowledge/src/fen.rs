/// Structural validation only. Does not establish reachability, legality,
/// repetition/chasing adjudication, or correspondence with an image/history.
pub fn validate_fen(fen: &str) -> Result<(), String> {
    let fields: Vec<_> = fen.split_whitespace().collect();
    if fields.len() != 6 {
        return Err("expected six FEN fields".into());
    }
    let ranks: Vec<_> = fields[0].split('/').collect();
    if ranks.len() != 10 {
        return Err("expected ten ranks".into());
    }
    let mut red_general = 0;
    let mut black_general = 0;
    for rank in ranks {
        let mut width = 0;
        for c in rank.chars() {
            match c {
                '1'..='9' => width += c.to_digit(10).unwrap_or(0),
                'k' => {
                    black_general += 1;
                    width += 1;
                }
                'K' => {
                    red_general += 1;
                    width += 1;
                }
                'r' | 'n' | 'b' | 'a' | 'c' | 'p' | 'R' | 'N' | 'B' | 'A' | 'C' | 'P' => width += 1,
                _ => {
                    return Err(format!(
                        "unsupported piece/digit {c:?}; canonical alphabet is rnbakcp"
                    ));
                }
            }
        }
        if width != 9 {
            return Err("each rank must cover nine intersections".into());
        }
    }
    if red_general != 1 || black_general != 1 {
        return Err("study positions require exactly one general per side".into());
    }
    if !matches!(fields[1], "w" | "b") || fields[2] != "-" || fields[3] != "-" {
        return Err("expected explicit w/b side and '- -' placeholders".into());
    }
    fields[4]
        .parse::<u32>()
        .map_err(|_| "invalid halfmove counter")?;
    if fields[5]
        .parse::<u32>()
        .map_err(|_| "invalid fullmove counter")?
        == 0
    {
        return Err("fullmove counter starts at one".into());
    }
    Ok(())
}

pub fn valid_coordinate_move(m: &str) -> bool {
    let b = m.as_bytes();
    b.len() == 4
        && (b'a'..=b'i').contains(&b[0])
        && b[1].is_ascii_digit()
        && (b'a'..=b'i').contains(&b[2])
        && b[3].is_ascii_digit()
        && b[..2] != b[2..]
}
