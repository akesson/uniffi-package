import XCTest
@testable import swift_cmd

final class swift_cmdTests: XCTestCase {
    func testExample() throws {
        // This is an example of a functional test case.
        // Use XCTAssert and related functions to verify your tests produce the correct
        // results.
        XCTAssertEqual(swift_cmd().text, "Hello, World!")
    }
}
