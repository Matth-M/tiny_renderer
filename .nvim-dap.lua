local dap = require("dap")

dap.adapters.lldb = {
	type = "executable",
	command = "/usr/bin/lldb-vscode", -- adjust as needed
	name = "lldb",
}

dap.configurations.rust = {
	{
		name = "tiny_renderer",
		type = "lldb",
		request = "launch",
		program = function()
			return vim.fn.getcwd() .. "/target/debug/tiny_renderer"
		end,
		cwd = "${workspaceFolder}",
		stopOnEntry = false,
	},
}
