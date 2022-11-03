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


use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::trace::multitrace::{MultiTrace, Trace};
use crate::core::general_context::GeneralContext;
use crate::core::language::hide::hideable::LifelineHideable;
use crate::core::language::syntax::interaction::Interaction;
use crate::process::abstract_proc::common::HibouSearchStrategy;
use crate::process::abstract_proc::manager::GenericProcessPriorities;
use crate::process::ana_proc::logic::anakind::{AnalysisKind, UseLocalAnalysis};
use crate::process::ana_proc::interface::filter::AnalysisFilter;
use crate::process::ana_proc::manager::AnalysisProcessManager;
use crate::process::ana_proc::interface::priorities::AnalysisPriorities;
use crate::process::ana_proc::logic::flags::{MultiTraceAnalysisFlags, TraceAnalysisFlags};
use crate::process::ana_proc::logic::verdicts::GlobalVerdict;

pub fn is_dead_local_analysis(gen_ctx : &GeneralContext,
                              co_localizations : &CoLocalizations,
                              parent_analysis_kind : &AnalysisKind,
                              use_locana : &UseLocalAnalysis,
                              interaction : &Interaction,
                              multi_trace : &MultiTrace,
                              flags : &mut MultiTraceAnalysisFlags) -> bool {
    match use_locana {
        UseLocalAnalysis::No => {
            // nothing
        },
        UseLocalAnalysis::Yes(only_front) => {
            for (canal_id, colocalized_lfs) in co_localizations.locs_lf_ids.iter().enumerate() {
                let canal_flags: &mut TraceAnalysisFlags = flags.canals.get_mut(canal_id).unwrap();
                let canal_trace: &Trace = multi_trace.get(canal_id).unwrap();
                // ***
                if canal_flags.dirty4local && canal_trace.len() > canal_flags.consumed {
                    let local_flags : MultiTraceAnalysisFlags = MultiTraceAnalysisFlags::new(vec![canal_flags.clone()], flags.rem_loop_in_sim, flags.rem_act_in_sim);
                    let local_multi_trace : MultiTrace = vec![canal_trace.clone()];
                    let local_interaction : Interaction;
                    {
                        let mut lfs_to_remove = gen_ctx.get_all_lfs_ids();
                        for lf_id in colocalized_lfs {
                            lfs_to_remove.remove( &lf_id );
                        }
                        local_interaction = interaction.hide(&lfs_to_remove);
                    }
                    let local_coloc = CoLocalizations::new(vec![colocalized_lfs.clone()]);
                    match perform_local_analysis(gen_ctx,local_coloc,parent_analysis_kind,local_interaction,local_multi_trace,local_flags,*only_front) {
                        GlobalVerdict::Fail => {
                            return true;
                        },
                        GlobalVerdict::WeakFail => {
                            return true;
                        },
                        _ => {}
                    }
                }
                // ***
                canal_flags.dirty4local = false;
            }
        }
    }
    return false;
}




fn perform_local_analysis(gen_ctx : &GeneralContext,
                          local_coloc : CoLocalizations,
                          parent_analysis_kind : &AnalysisKind,
                          local_interaction : Interaction,
                          local_multi_trace : MultiTrace,
                          local_flags : MultiTraceAnalysisFlags,
                          only_front : bool) -> GlobalVerdict {
    // ***
    let local_analysis_kind : AnalysisKind;
    match parent_analysis_kind {
        AnalysisKind::Simulate( sim_config ) => {
            if sim_config.sim_before {
                local_analysis_kind = AnalysisKind::Simulate(sim_config.clone());
            } else {
                local_analysis_kind = AnalysisKind::Prefix;
            }
        },
        _ => {
            local_analysis_kind = AnalysisKind::Prefix;
        }
    }
    // ***
    let new_gen_ctx= gen_ctx.clone();
    // ***
    let mut locana_filters : Vec<AnalysisFilter> = vec![];
    if only_front {
        locana_filters.push( AnalysisFilter::MaxProcessDepth(1) );
    }
    // ***
    let mut local_analysis_manager = AnalysisProcessManager::new(new_gen_ctx,
                                                                 local_coloc,
                                                                 local_multi_trace,
                                                                 HibouSearchStrategy::DFS,
                                                                 locana_filters,
                                                                 GenericProcessPriorities::Specific(AnalysisPriorities::default()),
                                                                 vec![],
                                                                 local_analysis_kind,
                                                                 UseLocalAnalysis::No,
                                                                 Some(GlobalVerdict::WeakPass)
    );
    let local_int_characs = local_interaction.get_characteristics();
    let (local_verdict,_) = local_analysis_manager.analyze(local_interaction,local_flags);
    return local_verdict;
}