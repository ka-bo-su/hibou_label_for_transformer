/*
Copyright 2020 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/




use std::collections::{HashMap, HashSet};
use crate::core::semantics::frontier::FrontierElement;
use crate::core::trace::TraceActionKind;
use crate::proc_refactoring::abstract_proc::AbstractStepKind;
use crate::proc_refactoring::ana_proc::conf::AnalysisConfig;
use crate::proc_refactoring::ana_proc::priorities::AnalysisPriorities;
use crate::process::hibou_process::SimulationStepKind;


pub enum AnalysisStepKind {
    Hide( HashSet<usize> ), // all the lifelines to hide
    Simulate(FrontierElement,HashMap<usize,SimulationStepKind>) // for each lifeline on which simulation is done, which kind (the other lifelines, if participating in the action, consume events)
}

impl AbstractStepKind<AnalysisConfig> for AnalysisStepKind {

    fn get_priority(&self, process_priorities: &AnalysisPriorities) -> i32 {
        match self {
            AnalysisStepKind::Hide(_) => {
                return 0;
            },
            AnalysisStepKind::Simulate(frt_elt,sim_map) => {
                let mut priority : i32 = 0;
                match frt_elt.act_kind {
                    TraceActionKind::Emission => {
                        priority += process_priorities.emission;
                    },
                    TraceActionKind::Reception => {
                        priority += process_priorities.reception;
                    }
                }
                priority += process_priorities.multi_rdv * ( frt_elt.target_actions.len() as i32);
                priority += process_priorities.in_loop * ( frt_elt.loop_depth as i32);
                priority += process_priorities.simu * (sim_map.len() as i32);
                // ***
                return priority;
            }
        }
    }

}