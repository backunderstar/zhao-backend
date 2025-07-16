pub fn update_err_msg(err_msg: &mut Option<String>, new_msg: String) {
    *err_msg = match err_msg.take() {
        Some(current_msg) => Some(format!("{}; {}", current_msg, new_msg)),
        None => Some(new_msg),
    };
}
