@main
public struct swift_cmd {
    public private(set) var text = "Hello, World!"

    public static func main() {
        print(swift_cmd().text)
    }
}
