const initialState = {
	lines: [],
	sendLine: '',
}

export default function WSEvent(state=initialState,action){
	switch (action.type){
		case 'ADD_LINE':
			let newLines = state.lines.slice()
			newLines.push(action.line)
			while (newLines.length > 20){
				newLines.shift()
			}
			return Object.assign({},state,{
				lines:newLines,
			})
		case 'SEND_LINE':
			return Object.assign({},state,{
				sendLine: action.line,
			})
		case 'SENT_LINE':
			return Object.assign({},state,{
				sendLine: '',
			})
		default:
			return state
	}
}