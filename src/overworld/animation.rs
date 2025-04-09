use macroquad::prelude::*;

pub struct AnimationStep {
    tex : Texture2D,
    frames : i32
}

pub struct Animation {
    _name : String,
    steps : Vec<AnimationStep>,
    total_frames : i32
}

impl AnimationStep {
    pub fn new(texture : Texture2D, frame_count : i32) -> AnimationStep {
        AnimationStep {
            tex : texture,
            frames : frame_count
        }
    }

    pub fn get_frame_count(&self) -> i32 {
        self.frames
    }

    pub fn get_texture(&self) ->  Texture2D {
        self.tex.clone()
    }
}

impl Animation {
    pub fn new(name : String) -> Animation {
        Animation {
            _name : name,
            steps : Vec::new(),
            total_frames : 0
        }
    }

    pub fn _add_step(&mut self, step : AnimationStep) {
        self.total_frames += step.get_frame_count();
        self.steps.push(step);
    }

    pub fn new_step(&mut self, tex : Texture2D, frame_count : i32) {
        self.total_frames += frame_count;
        self.steps.push(AnimationStep::new(tex, frame_count));
    }

    pub fn get_texture(&mut self, frame : i32) -> Texture2D {        
        let mut frame_current_count = 0;
        for val in &self.steps {
            frame_current_count += val.get_frame_count();
            if frame < frame_current_count {return val.get_texture()}
            
        }

        return self.get_texture(frame % self.total_frames);
    }

    pub fn get_total_frames(&self) -> i32 {
        self.total_frames
    }
}