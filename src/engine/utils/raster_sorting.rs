use crate::engine::projection::projection::ProjectiveScene;
use crate::engine::projection::raster::Raster;

impl ProjectiveScene {
    fn insertion_sort(vec: &mut [Raster]) {
        for i in 1..vec.len() {
            let mut j = i;
            while j > 0 && vec[j].z < vec[j - 1].z {
                vec.swap(j, j - 1);
                j -= 1;
            }
        }
    }

    fn qs_partition(vec: &mut [Raster], low: usize, high: usize) -> usize {
        let pivot = vec[high].z;
        let mut i = low;

        for j in low..high {
            if vec[j].z < pivot {
                vec.swap(i, j);
                i += 1;
            }
        }
        vec.swap(i, high);
        i
    }

    fn quicksort(vec: &mut [Raster], low: usize, high: usize) {
        if high - low + 1 < 20 {
            ProjectiveScene::insertion_sort(&mut vec[low..=high]);
        } else {
            if low < high {
                let pi = ProjectiveScene::qs_partition(vec, low, high);
                if pi > 0 {
                    ProjectiveScene::quicksort(vec, low, pi - 1);
                }
                ProjectiveScene::quicksort(vec, pi + 1, high);
            }
        }
    }

    pub fn sort_rasters(mut vec: Vec<Raster>) -> Vec<Raster> {
        if vec.len() > 1 {
            let len = vec.len();
            ProjectiveScene::quicksort(&mut vec, 0, len - 1);
        }
        vec
    }
}