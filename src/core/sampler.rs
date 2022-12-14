use crate::core::sample::Sample;

/////////////////////
// BEGIN INTERFACE //
/////////////////////

pub trait Sampler {
    fn next_sample(&self) -> Option<Sample>;
}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

////////////////////////
// END IMPLEMENTATION //
// BEGIN TESTS        //
////////////////////////

///////////////
// END TESTS //
///////////////
