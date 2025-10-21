// TODO: Create here the pipes and pass an arument so they aren't created on the other programs

use fast_chemical_synapse_rs::fast_chemical_synapse::FastChemicalSynapse;
use hindmarsh_rose_rs::hindmarsh_rose::{self, HindmarshRoseModel, HindmarshRoseRungeKutta};
use model_data_io::{
    DataArgument,
    data_writer::{DataWriter, DataWriterCsv},
};
fn main() {
    let filename = "hindmarsh-rose-single-exec-syn.csv".to_string();
    let goal = 20000.0;
    let time_increment = 0.0015;
    let downsample_rate = 100;

    let mut time_counter = 0.0;
    let mut downsample_counter = 0;

    let x = -1.3;
    let y = 1.0;
    let z = 1.0;

    let e = 3.0;
    let mu = 0.0021;
    let s = 4.0;
    let vh = 1.0;

    // let g_fast = 0.241;
    let g_fast = 0.046;
    let e_syn = -1.92;
    let s_fast = 0.44;
    let v_fast = -1.66;

    let fast_chemical_synapse_rs = FastChemicalSynapse::new(g_fast, e_syn, s_fast, v_fast);

    let model_derivatives_pre = hindmarsh_rose::ModelDerivativeVariables::new(x, y, z);
    let model_derivatives_pos = hindmarsh_rose::ModelDerivativeVariables::new(x, y, z);
    let temporal_variables_pre = hindmarsh_rose::ModelTemporalVariables::new(e, mu, s, vh);
    let temporal_variables_pos = hindmarsh_rose::ModelTemporalVariables::new(e, mu, s, vh);
    let mut hr_pre = HindmarshRoseRungeKutta::new(
        model_derivatives_pre,
        temporal_variables_pre,
        time_increment,
    );
    let mut hr_pos = HindmarshRoseRungeKutta::new(
        model_derivatives_pos,
        temporal_variables_pos,
        time_increment,
    );

    let mut writer = DataWriterCsv::new(filename);

    while time_counter < goal {
        hr_pre.calculate_hindmarsh_rose();
        hr_pos.calculate_hindmarsh_rose();

        time_counter += time_increment;
        let (x_pre, _, _) = hr_pre.get_model_info();
        let (x_pos, _, _) = hr_pos.get_model_info();
        hr_pos.update_i_syn(fast_chemical_synapse_rs.calculate(x_pre, x_pos));
        if downsample_counter == downsample_rate {
            downsample_counter = 0;
            let x_arg_pre = DataArgument::new("x_pre".to_string(), x_pre);
            let time_arg_pre = DataArgument::new("x_pre_time".to_string(), time_counter);
            let x_arg_pos = DataArgument::new("x_pos".to_string(), x_pos);
            let time_arg_pos = DataArgument::new("x_pos_time".to_string(), time_counter);
            writer.inter_loop(&vec![x_arg_pos, time_arg_pos, x_arg_pre, time_arg_pre]);
        }

        downsample_counter += 1;
    }
    writer.after_loop();
}
