import { DebugSession } from 'vscode-debugadapter';
import { Command } from 'commander';
import { LogFn } from '@aztec/foundation/log';

class NoirJsDebugAdapter extends DebugSession {
	constructor() {
		super();
	}
}

export function startDap(program: Command, name = 'dap', log: LogFn = () => {}): Command {
  return program
    .command(name)
    .description('Start a debug adapter')
    .action(() => {
    	log('Starting debug adapter server');
    	DebugSession.run(NoirJsDebugAdapter);
    });
}
