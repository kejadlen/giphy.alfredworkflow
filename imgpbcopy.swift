// Ported from http://www.alecjacobson.com/weblog/?p=3816

import Foundation
import Cocoa

let args = CommandLine.arguments
let path = args[1]
let image = NSImage(contentsOfFile: path)!
let pasteboard = NSPasteboard.general()

pasteboard.clearContents()
pasteboard.writeObjects([image])
