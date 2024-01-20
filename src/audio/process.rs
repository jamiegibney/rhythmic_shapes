//! Module for the audio processing callback.

use super::{model::AudioModel, voice::*};
use crate::prelude::*;

pub const MAX_BLOCK_SIZE: usize = 64;

pub fn process(audio: &mut AudioModel, buffer: &mut Buffer) {
    let buffer_len = buffer.len_frames();

    let mut next_event = audio.note_receiver.try_recv().ok();

    let voice_handler = &mut audio.voice_handler;

    let mut block_start: usize = 0;
    let mut block_end = MAX_BLOCK_SIZE.min(buffer_len);

    // audio generators
    while block_start < buffer_len {
        // first, handle incoming events.
        'events: loop {
            match next_event {
                // if the event is now (or before the block), match
                // the event and handle its voice accordingly.
                Some(event) if (event.timing() as usize) <= block_start => {
                    match event {
                        NoteEvent::NoteOn { .. } => {
                            let note = event.note();
                            voice_handler.start_voice(note);
                        }
                        NoteEvent::NoteOff { .. } => {
                            unimplemented!("note off events are not implemented for this project");
                        }
                    }

                    // then obtain the next event and loop again
                    next_event = audio.note_receiver.try_recv().ok();
                }
                // if the event exists within this block, set the next block
                // to start at the event and continue processing the block
                Some(event) if (event.timing() as usize) < block_end => {
                    block_end = event.timing() as usize;
                    break 'events;
                }
                _ => break 'events,
            }
        }

       let block_len = block_end - block_start;

        voice_handler.process_block(buffer, block_start, block_end);
        voice_handler.terminate_finished_voices();

        block_start = block_end;
        block_end = (block_end + MAX_BLOCK_SIZE).min(buffer_len);
    }

    audio.set_callback_timer();
}
