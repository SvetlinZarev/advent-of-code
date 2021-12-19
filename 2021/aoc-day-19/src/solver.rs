use crate::math::{distance_sq, is_same_vector, rotate, subtract};
use crate::{Int, Map, Point, Set};

const COMBINATIONS_OF_2_FROM_12: usize = 66;

pub(crate) fn calculate_distances(input: &[Vec<Point>]) -> Vec<Map<Int, Vec<(Point, Point)>>> {
    let mut distances = vec![Map::default(); input.len()];

    for (idx, scanner) in input.iter().enumerate() {
        for i in 0..scanner.len() {
            for j in i + 1..scanner.len() {
                let a = scanner[i];
                let b = scanner[j];
                let d = distance_sq(a, b);

                distances[idx]
                    .entry(d)
                    .and_modify(|v: &mut Vec<(Point, Point)>| v.push((a, b)))
                    .or_insert(vec![(a, b)]);
            }
        }
    }
    distances
}

pub(crate) fn calculate_corrections(
    mut distances: Vec<Map<Int, Vec<(Point, Point)>>>,
    input: &[Vec<Point>],
) -> Vec<(u8, Point)> {
    let mut normalized = vec![None; input.len()];
    normalized[0] = Some((0, (0, 0, 0)));

    let mut remaining = input.len() - 1;

    // Loop until we have processed all scanners
    while remaining > 0 {
        // skip the scanner if it has already been processed
        for scanner_idx in 1..input.len() {
            if normalized[scanner_idx].is_some() {
                continue;
            }

            // The current scanner has not been fixed. Check if it has at
            // least 12 common points with at least one of the processed
            // scanners
            for norm_idx in 0..input.len() {
                // Do not use these distances as anchor, because those
                // points have not been normalized yet
                if normalized[norm_idx].is_none() {
                    continue;
                }

                let mut common_points = vec![];

                let anchor_dist = &distances[norm_idx];
                let scanner_dist = &distances[scanner_idx];

                // Find the points that have the same distances between them
                for (dist, uncalibrated_points) in scanner_dist.iter() {
                    if let Some(anchor_vector) = anchor_dist.get(dist) {
                        common_points.push((anchor_vector, uncalibrated_points));
                    }
                }

                // If we have found at least 12 points, we can calculate the rotation and the translation
                if common_points.len() >= COMBINATIONS_OF_2_FROM_12 {
                    if let Some((rot, diff)) = detect_rotation(&common_points) {
                        normalized[scanner_idx] = Some((rot, diff));

                        // Fix the anchor points for the new anchor
                        distances[scanner_idx].iter_mut().for_each(|(_, points)| {
                            points.iter_mut().for_each(|(a, b)| {
                                *a = rotate(a.clone(), rot);
                                *b = rotate(b.clone(), rot);

                                *a = (a.0 - diff.0, a.1 - diff.1, a.2 - diff.2);
                                *b = (b.0 - diff.0, b.1 - diff.1, b.2 - diff.2);
                            });
                        });

                        remaining -= 1;
                        break;
                    };
                }
            }
        }
    }

    normalized.into_iter().map(|x| x.unwrap()).collect()
}

fn detect_rotation(
    common_beacons: &[(&Vec<(Point, Point)>, &Vec<(Point, Point)>)],
) -> Option<(u8, Point)> {
    let mut rotations = Map::<u8, Set<Point>>::default();

    //TODO speed up detection by reusing the first detected rotation
    for (anchors, to_normalize) in common_beacons.iter() {
        for &anchor in anchors.iter() {
            for &vector in to_normalize.iter() {
                debug_assert_eq!(
                    distance_sq(anchor.0, anchor.1),
                    distance_sq(vector.0, vector.1)
                );

                for rot in 0..24 {
                    let p1 = rotate(vector.0, rot);
                    let p2 = rotate(vector.1, rot);

                    if is_same_vector(anchor, (p1, p2)) {
                        let points = rotations.entry(rot).or_default();
                        points.insert(vector.0);
                        points.insert(vector.1);

                        // TODO: why are there only 11 points for some scanners ?
                        if points.len() >= 11 {
                            let diff = subtract(p1, anchor.0);
                            return Some((rot, diff));
                        }
                    }
                }
            }
        }
    }

    None
}
