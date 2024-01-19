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

    pub fn intersects(&self, other: &Rect) -> bool {
        if self.left() > other.right()
            || self.right() < other.left()
            || self.bottom() > other.top()
            || self.top() < other.bottom()
        {
            return false;
        }
        true
    }

    pub fn unobstructed_rects(&self, other_rect: &[&Rect]) -> Vec<Rect> {
        // convert to a vec and remove any that don't overlap self
        let mut sorted_rects: Vec<&Rect> = other_rect
            .into_iter()
            .cloned()
            .filter(|rect| rect.intersects(self))
            .collect();

        // sort them like shingles on a roof
        // then we can just check if there is space between the end of one and the start of the next
        sorted_rects.sort_unstable_by(
            // descending order
            |rect_a, rect_b| {
                rect_b.top().cmp(&rect_a.top()) // by the first point on each
            },
        );

        // make a list of all points where rects can change
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

        // convert to an iterator and filter out points before or after self
        let points_iter = points
            .into_iter()
            .filter(|point| &self.left() <= point && point <= &self.right());

        // we can store rects in this until we find where they end.
        let mut active: Vec<(i64, i64, i64)> = Vec::new(); // (left_x, top_y, bottom_y)
                                                           // we can put out finished rects in here
        // a list of rects which will be returned
        let mut unique_rects: Vec<Rect> = Vec::new();

        for point in points_iter {
            // Step #1.
            // we need to find all the gaps (space between shingles) at the current point

            // we can store the here
            let mut gaps: Vec<(i64, i64)> = Vec::new(); // (top_y, bottom_y)

            // this will hold the bottom position of the last shingle
            // it can start at the top of self
            let mut last_pos: i64 = self.top();

            // convert to a iter and filter out any rects which don't intersect the current slice
            let mut rect_iter = sorted_rects
                .iter()
                .filter(|rect| rect.left() <= point && point <= rect.right());

            loop {
                // get the next rect
                let rect = match rect_iter.next() {
                    Some(value) => value,
                    None => {
                        // if there is a gap between the bottom of the last rect and self
                        if last_pos > self.bottom() {
                            // we can add it to gaps
                            gaps.push((last_pos, self.bottom()));
                        }
                        break;
                    }
                };

                // if there is a gap greater then zero
                if rect.top() < last_pos {
                    // add it
                    gaps.push((last_pos, rect.top() + 1));
                }

                // update last_pos
                last_pos = last_pos.min(rect.bottom() - 1);
            }

            // Step #2.
            // Now that we have a list of gaps we check if any of our active sections are blocked

            // we may end up with new rects if one is only partially blocked
            // this can store them until we collect our iterator
            let mut new_active: Vec<(i64, i64, i64)> = Vec::new();

            // ensure that we don't block larger rects if they arn't unique
            // descending order, the lower the number the lager the section
            active.sort_unstable_by(|act_a, act_b| act_b.0.cmp(&act_a.0));

            active = active
                .iter() // into_ if you arn't looking for unique rects
                .cloned() // not needed if you used into
                .filter(|act| -> bool {
                    for gap in gaps.iter() {
                        if gap.0 >= act.1 && act.2 >= gap.1 {
                            // if it fits within a gap we are good to go
                            return true;
                        }
                    }
                    // if we have not exited yet, act is blocked

                    // we can add it to the list of rects
                    unique_rects.push(Rect::new(
                        act.0,
                        act.2,
                        point - act.0, // if it is blocked we are actually one after where it ended
                        // but because everything is inclusive it works out
                        (act.1 - act.2) + 1,
                    ));

                    // there my however be thinner rects that can continue
                    for gap in gaps.iter().filter(|gap| gap.0 <= act.1 || act.2 <= gap.1) {
                        // ensure that the gap is unique
                        'outer: for section in active.iter() {
                            if gap.0 == section.1 && gap.1 == section.2 {
                                continue 'outer;
                            }
                        }
                        // even with sections just created
                        'outer: for section in new_active.iter() {
                            if gap.0 == section.1 && gap.1 == section.2 {
                                continue 'outer;
                            }
                        }
                        // I guess it's unique
                        new_active.push((act.0, gap.0.min(act.1), gap.1.max(act.2)));
                    }

                    // drop the current from active
                    return false;
                })
                .collect();

            // merge the new sections
            active.append(&mut new_active);

            // Step #3.
            // add a sections for any new gaps
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

        // now that we have checked all points of interest
        // active will only contain sections that are never obstructed
        for act in active.iter() {
            // so they end at self.right()
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
