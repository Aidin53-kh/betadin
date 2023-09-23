fn read_file(path) {
    if typeof path != "string" {
        return {
            is_ok: false,
            error: "the path most be a string",
            content: null
        };
    }

    const content = std::fs::read_file(path);

    return {
        is_ok: true,
        error: null,
        content: content
    };
}