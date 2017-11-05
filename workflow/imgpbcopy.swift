import Cocoa

let path = CommandLine.arguments[1]
let url = URL(fileURLWithPath: path).absoluteURL as NSURL

let pb = NSPasteboard.general
pb.declareTypes([NSPasteboard.PasteboardType.fileContents], owner: nil)
pb.writeObjects([url])
