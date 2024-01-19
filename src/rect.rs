#[derive(Debug)]
pub struct Rect {
    x: i64,
    y: i64,
    width: i64,
    height: i64,
}

impl Rect {
    pub fn new(x: i64, y: i64, width: i64, height: i64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn left(&self) -> i64 {
        self.x
    }

    pub fn right(&self) -> i64 {
        self.x + self.width - 1
    }

    pub fn top(&self) -> i64 {
        self.y + self.height - 1
    }

    pub fn bottom(&self) -> i64 {
        self.y
    }

    pub fn unobstructed_rects(&self, other_rect: &[&Rect]) -> Vec<Rect> {
        let mut sorted_rects: Vec<&Rect> = other_rect.into_iter().cloned().collect();

        sorted_rects.sort_unstable_by(
            // descending order
            |rect_a, rect_b| {
                rect_b.top().cmp(&rect_a.top()) // by the first point on each
            },
        );

        let mut points: Vec<i64> = vec![self.left()];

        for rect in sorted_rects.iter() {
            // gaps might close here
            points.push(rect.left());

            // gaps might open here
            points.push(rect.right() + 1);
        }

        // smallest first
        points.sort();
        // remove duplicates
        points.dedup();

        let points_iter = points
            .into_iter()
            .filter(|point| &self.left() <= point && point <= &self.right());

        let mut active: Vec<(i64, i64, i64)> = Vec::new();

        let mut unique_rects: Vec<Rect> = Vec::new();

        for point in points_iter {
            // Step #1.

            let mut gaps: Vec<(i64, i64)> = Vec::new(); // (top_y, bottom_y)

            let mut last_pos: i64 = self.top();

            let mut rect_iter = sorted_rects
                .iter()
                .filter(|rect| rect.left() <= point && point <= rect.right());

            loop {
                let rect = match rect_iter.next() {
                    Some(value) => value,
                    None => {
                        if last_pos > self.bottom() {
                            gaps.push((last_pos, self.bottom()));
                        }
                        break;
                    }
                };

                if rect.top() < last_pos {
                    gaps.push((last_pos, rect.top() + 1));
                }

                last_pos = last_pos.min(rect.bottom() - 1);
            }

            // Step #2.

            let mut new_active: Vec<(i64, i64, i64)> = Vec::new();

            // descending order, the lower the number the larger the section
            active.sort_unstable_by(|act_a, act_b| act_b.0.cmp(&act_a.0));

            active = active
                .iter()
                .cloned()
                .filter(|act| -> bool {
                    for gap in gaps.iter() {
                        if gap.0 >= act.1 && act.2 >= gap.1 {
                            return true;
                        }
                    }

                    unique_rects.push(Rect::new(act.0, act.2, point - act.0, (act.1 - act.2) + 1));

                    'outer: for gap in gaps.iter().filter(|gap| gap.0 <= act.1 || act.2 <= gap.1) {

                        let top_pos = gap.0.min(act.1);
                        let bot_pos = gap.1.max(act.2);

                        for section in active.iter() {
                            if top_pos == section.1 && bot_pos == section.2 {
                                continue 'outer;
                            }
                        }
                        for section in new_active.iter() {
                            if top_pos == section.1 && bot_pos == section.2 {
                                continue 'outer;
                            }
                        }
                        // I guess it's unique
                        new_active.push((act.0, top_pos, bot_pos));
                    }

                    // drop the current from active
                    return false;
                })
                .collect();

            // merge the new sections
            active.append(&mut new_active);

            // Step #3.

            'outer: for gap in gaps.iter() {
                for section in active.iter() {
                    if gap.0 == section.1 && gap.1 == section.2 {
                        continue 'outer;
                    }
                }
                // I guess it's unique
                active.push((point, gap.0, gap.1));
            }
        }

        for act in active.iter() {
            unique_rects.push(Rect::new(
                act.0,
                act.2,
                (self.right() - act.0) + 1,
                (act.1 - act.2) + 1,
            ))
        }

        // Quod Erat Demonstrandum
        unique_rects
    }
}
