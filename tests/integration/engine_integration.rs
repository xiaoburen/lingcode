// This file contains integration tests for the engine module of the input method framework. 

#[cfg(test)]
mod tests {
    use super::*;
    use engine::Engine; // Assuming the Engine struct is defined in the engine module

    #[test]
    fn test_engine_initialization() {
        let engine = Engine::new();
        assert!(engine.is_initialized());
    }

    #[test]
    fn test_pinyin_input() {
        let engine = Engine::new();
        let result = engine.process_input("ni hao");
        assert_eq!(result, vec!["你好"]);
    }

    #[test]
    fn test_double_pinyin_input() {
        let engine = Engine::new();
        let result = engine.process_double_pinyin("nh");
        assert_eq!(result, vec!["你好"]);
    }

    #[test]
    fn test_invalid_input() {
        let engine = Engine::new();
        let result = engine.process_input("invalid_input");
        assert!(result.is_empty());
    }
}