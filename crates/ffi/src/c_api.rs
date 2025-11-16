// This file defines the C language interface for the Rust input method framework. 

extern "C" {
    // Initialize the input method framework
    pub fn init_input_method();

    // Finalize the input method framework
    pub fn finalize_input_method();

    // Process input string and return the candidates
    pub fn process_input(input: *const c_char) -> *mut c_char;

    // Set the language mode (e.g., simplified, traditional)
    pub fn set_language_mode(mode: c_int);

    // Set the input method type (e.g., pinyin, double pinyin)
    pub fn set_input_method_type(method_type: c_int);
}