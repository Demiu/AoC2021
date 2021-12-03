// (unmodified input, line length)
type SolverInput<'a> = (&'a [u8], usize);

pub fn parse_input<'a>(file_bytes: &'a [u8]) -> Option<SolverInput<'a>> {
    let mut i = 0;
    while let Some(ch) = file_bytes.get(i) {
        match ch {
            b'0' | b'1' => i += 1,
            _ => {
                if i == 0 {
                    return None;
                } else {
                    return Some((file_bytes, i));
                }
            }
        }
    }
    return None;
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let mut counts = vec![0; input.1];
    let mut lines = 0;
    let mut linepos = 0;
    let mut it = input.0.iter();
    while let Some(ch) = it.next() {
        match ch {
            b'0' | b'1' => {
                if *ch == b'1' {
                    counts[linepos] += 1
                };
                if linepos == 0 {
                    lines += 1
                };
                linepos += 1;
            }
            b'\n' => linepos = 0,
            _ => unreachable!(),
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for count in counts {
        gamma *= 2;
        epsilon *= 2;
        if count * 2 > lines {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    return gamma * epsilon;
}
