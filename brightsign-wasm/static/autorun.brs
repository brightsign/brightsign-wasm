Sub Main()
	REM Create a web server to fetch the wasm file from
	server = CreateObject("roHttpServer", { port: 8999 })
	server.AddGetFromFolder({folder: "SD:"})
	REM create the html widget
	vm = CreateObject("roVideoMode")
	rect = CreateObject("roRectangle", 0, 0, vm.GetResX(), vm.GetResY())
	msg_port = CreateObject("roMessagePort")
	params = {
		port: msg_port
		url: "http://0.0.0.0:8999/index.html"
		javascript_enabled: true
	}
	html = CreateObject("roHtmlWidget", rect, params)
	html.Show()
	while true
		wait(0, msg_port)
	end while
End Sub
