vim.g.dbs = {
	{
		name = "foodie",
		url = function()
			return vim.fn.readfile(".db-url")[1]
		end,
	},
}

require("formatter").setup({
	filetype = {
		rust = {
			"rustfmt +nightly --edition 2021",
			{
				exe = "leptosfmt",
				args = { "--stdin" },
				cond = function()
					return vim.fs.root(0, "leptosfmt.toml") ~= nil
				end,
			},
		},
	},
})
