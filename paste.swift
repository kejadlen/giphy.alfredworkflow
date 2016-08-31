import Cocoa

var url_arg = Process.arguments[1]
print ( url_arg )

var pb = NSPasteboard(name: NSGeneralPboard)

if let url = NSURL(string: url_arg) {
    if let data = NSData(contentsOfURL: url) {
        var image =  NSImage(data: data)
        let fileName = url.lastPathComponent
        var saveURL = NSURL(string: "file://" + NSTemporaryDirectory())
        saveURL = saveURL?.URLByAppendingPathComponent(fileName!)
        let data = NSData(contentsOfURL: url)
        data?.writeToURL(saveURL!, atomically: true)
        
        pb.declareTypes([NSFilenamesPboardType], owner: nil)
        pb.writeObjects([saveURL!])
        
    }
}

