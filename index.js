if (typeof localStorage === "undefined" || localStorage === null)
  module.exports = require("./native/index")
else
  module.exports = require("./web/index")
