const { DebugSession } = require('vscode-debugadapter');
const { EventEmitter } = require('events');

class NoirJsDebugAdapter extends DebugSession {
	constructor() {
		super();
	}
}

function startDap() {
	DebugSession.run(NoirJsDebugAdapter);	
}

module.exports = {
	startDap: startDap
}
