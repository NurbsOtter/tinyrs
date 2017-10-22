import React from 'react'
import { connect } from 'react-redux'

class LineBuffer extends React.Component{
	constructor(){
		super()
		this.state = {
			lines: []
		}
	}
	render(){
		console.log(this.props)
		return (
			<div>
				{this.props.lines.map((line,index) => {
					return (
						<div key={index}>{line}</div>
					)
				})}
			</div>
		)
	}
}
function mapStateToProps(state){
	return{
		lines:state.lines
	}
}
LineBuffer = connect(mapStateToProps)(LineBuffer)
export default LineBuffer