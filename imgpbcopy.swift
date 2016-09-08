// Ported from http://www.alecjacobson.com/weblog/?p=3816

import Foundation
import Cocoa

let args = Process.arguments
let path = args[1]
let image = NSImage(contentsOfFile: path)!
let pasteboard = NSPasteboard.generalPasteboard()

pasteboard.clearContents()
pasteboard.writeObjects([image])
