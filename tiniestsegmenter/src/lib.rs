mod weights;
use weights::*;

#[derive(Debug)]
pub enum TokenizeError {
    InvalidChar,
}

#[rustfmt::skip]
fn get_ctype(ch: char) -> char {
    match ch {
        '一' | '二' | '三' | '四' | '五' | '六' | '七' | '八' | '九' | '十' | '百' | '千' | '万' | '億' | '兆' => 'M',
        '々' | '〆' | 'ヵ' | 'ヶ' | '一'..='龠' => 'H',
        'ー' | 'ｰ' | '\u{FF9E}' | 'ァ'..='ヴ' | 'ｱ'..='ﾝ' => 'K',
        'ぁ'..='ん' => 'I',
        'a'..='z' | 'A'..='Z' | 'ａ'..='ｚ' | 'Ａ'..='Ｚ' => 'A',
        '0'..='9' | '０'..='９' => 'N',
        _ => 'O',
    }
}

fn push_char(s: &mut String, w: &Weight) -> Result<(), TokenizeError> {
    // Push the character from a Weight struct onto String s.
    // This should never happen, but if we try to push anything
    // other than char onto the string, we should throw an error
    match w {
        Weight::CHAR(ch) => {
            s.push(*ch);
            Ok(())
        }
        _ => Err(TokenizeError::InvalidChar),
    }
}

pub fn tokenize(s: &str) -> Result<Vec<String>, TokenizeError> {
    let mut result: Vec<String> = vec![];

    // If the input string is empty, we just return the empty Vec,
    // otherwise we can continue to tokenize the string as usual.
    if s.len() > 0 {
        let seg: Vec<Weight> = [Weight::B3, Weight::B2, Weight::B1]
            .into_iter()
            .chain(s.chars().map(Weight::CHAR))
            .chain([Weight::E1, Weight::E2, Weight::E3])
            .collect();

        let ctype: Vec<Weight> = [Weight::CHAR('O'), Weight::CHAR('O'), Weight::CHAR('O')]
            .into_iter()
            .chain(s.chars().map(|ch| Weight::CHAR(get_ctype(ch))))
            .chain([Weight::CHAR('O'), Weight::CHAR('O'), Weight::CHAR('O')])
            .collect();

        let mut word = String::new();
        push_char(&mut word, &seg[3])?;

        let mut p = [Weight::CHAR('U'), Weight::CHAR('U'), Weight::CHAR('U')];

        for i in 4..seg.len() - 3 {
            let mut score = BIAS;
            let w = &seg[i - 3..i + 3];
            let c = &ctype[i - 3..i + 3];

            score += up1(&p[0]);
            score += up2(&p[1]);
            score += up3(&p[2]);
            score += bp1(&p[0], &p[1]);
            score += bp2(&p[1], &p[2]);
            score += uw1(&w[0]);
            score += uw2(&w[1]);
            score += uw3(&w[2]);
            score += uw4(&w[3]);
            score += uw5(&w[4]);
            score += uw6(&w[5]);
            score += bw1(&w[1], &w[2]);
            score += bw2(&w[2], &w[3]);
            score += bw3(&w[3], &w[4]);
            score += tw1(&w[0], &w[1], &w[2]);
            score += tw2(&w[1], &w[2], &w[3]);
            score += tw3(&w[2], &w[3], &w[4]);
            score += tw4(&w[3], &w[4], &w[5]);
            score += uc1(&c[0]);
            score += uc2(&c[1]);
            score += uc3(&c[2]);
            score += uc4(&c[3]);
            score += uc5(&c[4]);
            score += uc6(&c[5]);
            score += bc1(&c[1], &c[2]);
            score += bc2(&c[2], &c[3]);
            score += bc3(&c[3], &c[4]);
            score += tc1(&c[0], &c[1], &c[2]);
            score += tc2(&c[1], &c[2], &c[3]);
            score += tc3(&c[2], &c[3], &c[4]);
            score += tc4(&c[3], &c[4], &c[5]);
            score += uq1(&p[0], &c[0]);
            score += uq2(&p[1], &c[1]);
            score += uq3(&p[2], &c[2]);
            score += bq1(&p[1], &c[1], &c[2]);
            score += bq2(&p[1], &c[2], &c[3]);
            score += bq3(&p[2], &c[1], &c[2]);
            score += bq4(&p[2], &c[2], &c[3]);
            score += tq1(&p[1], &c[0], &c[1], &c[2]);
            score += tq2(&p[1], &c[1], &c[2], &c[3]);
            score += tq3(&p[2], &c[0], &c[1], &c[2]);
            score += tq4(&p[2], &c[1], &c[2], &c[3]);

            // Rotate the array
            p.swap(0, 1);
            p.swap(1, 2);
            if score > 0 {
                result.push(word.clone());
                word.clear();
                p[2] = Weight::CHAR('B');
            } else {
                p[2] = Weight::CHAR('O');
            }
            push_char(&mut word, &seg[i])?;
        }

        result.push(word.clone());
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::tokenize;

    #[test]
    #[rustfmt::skip]
    fn test_tokenize() {
        assert_eq!(tokenize("キラーアプリを考える").unwrap(), ["キラーアプリ", "を", "考える"]);
        assert_eq!(tokenize("TinySegmenterはJavascriptだけ書かれた極めてコンパクトな日本語分かち書きソフトウェアです。").unwrap(), ["TinySegmenter", "は", "Javascript", "だけ", "書か", "れ", "た", "極め", "て", "コンパクト", "な", "日本", "語分", "かち", "書き", "ソフトウェア", "です", "。"]);
    }
}
