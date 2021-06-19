import init, {build_chaos_bag, draw_bag, ChaosBag, Token} from './pkg/ahlcg_bag_sim.js'

const AutoFail = -128;
const AutoSuccess = 127;

document.addEventListener("DOMContentLoaded", 
	async function() { 
		await init();
		
		addSliderEventListener('skull-value', 'skull-value-out');
		addSliderEventListener('cultist-value', 'cultist-value-out');
		addSliderEventListener('tablet-value', 'tablet-value-out');
		addSliderEventListener('elder-thing-value', 'elder-thing-value-out');
		addSliderEventListener('elder-sign-value', 'elder-sign-value-out');
		
		var simulation_btn = document.getElementById('run-simulation-btn');
		simulation_btn.addEventListener('click', buildChaosBag);
		//simulation_btn.addEventListener('click', draw);
		
		var auto_succ_chk = document.getElementById('elder-sign-auto-success-checkbox');
		auto_succ_chk.addEventListener('click', toggleElderSignState);
		
		toggleElderSignState();
});

var myChart;
function buildPlot(probabilities) {
	
	var labels = [];
	var values = [];
	
	var cummulative = 0;
	
	var haveAutoSucceed = false;
	for (let i = probabilities.length - 1; i >= 0; i--) {
		if (probabilities[i][0] == AutoFail) {
			labels[labels.length - 1] = labels[labels.length - 1] + ' or greater';
			break;
		}
		if (probabilities[i][0] == AutoSuccess) {
			haveAutoSucceed = true;
			cummulative += probabilities[i][1] * 100;
			continue;
		}
		
		var s = String(-probabilities[i][0]);
		if (probabilities[i][0] < 0) {
			s = '+' + s;
		}
		if (haveAutoSucceed) {
			s = s + ' or less';
			haveAutoSucceed = false;
		}
		
		labels.push(s);
		cummulative += probabilities[i][1] * 100;
		values.push(cummulative);
	}
	
	var ctx = document.getElementById('probability-chart').getContext('2d');
	if (myChart) {
		myChart.destroy();
	}
	
	myChart = new Chart(ctx, {
		type: 'bar',
		data: {
			labels: labels,
			datasets: [{
				data: values,
				backgroundColor: [
					'rgba(0, 166, 85, 0.5)',
				],
				barPercentage: 1.0,
				categoryPercentage: 1.0,
			}]
		},
		options: {
			scales: {
				y: {
					title: {
						display: true,
						text: "% Chance to Pass",
					},
					grid: {
						display: true,
					},
					min: 0,
					max: 100,
				},
				x: {
					grid: {
						display: false,
					},
					title: {
						display: true,
						text: "Relative skill value",
					},
				},
			},
			plugins: {
				legend: {
					display: false,
				},
				tooltip: {
					mode: 'x',
					intersect: false,
					displayColors: false,
					callbacks: {
						title: (toolTipItems, data) => {
							let title = 'Relative skill: ' + String(toolTipItems[0].label); // uses the x value of this point as the title
							return title;
						},
						label:  (toolTipItem, data) => {
							let label = 'Chance to pass: ' + toolTipItem.raw.toPrecision(4) + '%';
							return label;
						}
					},
				},
			},
			interaction: {
				mode: 'x',
				intersect: false,
			},
		},
	});
}


function toggleElderSignState() {
	var auto_succ_chk = document.getElementById('elder-sign-auto-success-checkbox');
	var elder_sign_range = document.getElementById('elder-sign-value');
	var elder_sign_val = document.getElementById('elder-sign-value-out');
	
	let enabled = auto_succ_chk.checked;
	
	if (enabled) {
		elder_sign_range.disabled = true;
		elder_sign_val.disabled = true;
	} else {
		elder_sign_range.disabled = false;
		elder_sign_val.disabled = false;
	}
}

function addSliderEventListener(slider_id, span_id) {
		var slider = document.getElementById(slider_id);
		var span = document.getElementById(span_id);
		
		slider.addEventListener('input', updateValueBuilder(span));
		span.textContent = slider.value;
}

function updateValueBuilder(output) {
	return function(input) {
		output.textContent = input.target.value;
	}
}

function lookupCount(id_prefix, min, max) {
	var i;
	var ret = 0;
	
	for (i = min; i <= max; i++) {
		let id = id_prefix + i.toString();
		var btn = document.getElementById(id);
		if (btn.checked) {
			ret = i;
			break;
		}
	}
	
	return ret;
}

function buildChaosBag() {
	// let bag = new Map();
	// bag['plus_one_cnt'] = lookupCount('plus-one-btn-radio',0,2);
	// bag['zero_cnt'] = lookupCount('zero-btn-radio',0,2);
	// bag['minus_one_cnt'] = lookupCount('minus-one-btn-radio',0,2);
	// bag['minus_two_cnt'] = lookupCount('minus-two-btn-radio',0,2);
	// bag['minus_three_cnt'] = lookupCount('minus-three-btn-radio',0,2);
	// bag['minus_four_cnt'] = lookupCount('minus-four-btn-radio',0,2);
	// bag['minus_five_cnt'] = lookupCount('minus-five-btn-radio',0,2);
	// bag['minus_six_cnt'] = lookupCount('minus-six-btn-radio',0,2);
	// bag['minus_seven_cnt'] = lookupCount('minus-seven-btn-radio',0,2);
	// bag['minus_eight_cnt'] = lookupCount('minus-eight-btn-radio',0,2);

	// bag['skull_cnt'] = lookupCount('skull-btn-radio',0,2);
	// bag['skull_val'] = parseInt(document.getElementById('skull-value').value);

	// bag['cultist_cnt'] = lookupCount('cultist-btn-radio',0,2);
	// bag['cultist_val'] = parseInt(document.getElementById('cultist-value').value);

	// bag['tablet_cnt'] = lookupCount('tablet-btn-radio',0,2);
	// bag['tablet_val'] = parseInt(document.getElementById('tablet-value').value);

	// bag['elder_thing_cnt'] = lookupCount('elder-thing-btn-radio',0,2);
	// bag['elder_thing_val'] = parseInt(document.getElementById('elder-thing-value').value);

	// bag['elder_sign_cnt'] = lookupCount('elder-sign-btn-radio',0,1);
	// bag['elder_sign_val'] = parseInt(document.getElementById('elder-sign-value').value);
	// bag['elder_sign_auto'] = document.getElementById('elder-sign-auto-success-checkbox').checked;

	// bag['auto_fail_cnt'] = lookupCount('auto-fail-btn-radio',0,1);

	// bag['bless_cnt'] = lookupCount('bless-btn-radio',0,10);
	// bag['curse_cnt'] = lookupCount('curse-btn-radio',0,10);

	// console.log(bag);
	
	let bag = build_chaos_bag();
	bag.set_token_count(Token['PlusOne'], lookupCount('plus-one-btn-radio',0,2));
	bag.set_token_count(Token['Zero'], lookupCount('zero-btn-radio',0,2));
	bag.set_token_count(Token['MinusOne'], lookupCount('minus-one-btn-radio',0,2));
	bag.set_token_count(Token['MinusTwo'], lookupCount('minus-two-btn-radio',0,2));
	bag.set_token_count(Token['MinusThree'], lookupCount('minus-three-btn-radio',0,2));
	bag.set_token_count(Token['MinusFour'], lookupCount('minus-four-btn-radio',0,2));
	bag.set_token_count(Token['MinusFive'], lookupCount('minus-five-btn-radio',0,2));
	bag.set_token_count(Token['MinusSix'], lookupCount('minus-six-btn-radio',0,2));
	bag.set_token_count(Token['MinusSeven'], lookupCount('minus-seven-btn-radio',0,2));
	bag.set_token_count(Token['MinusEight'], lookupCount('minus-eight-btn-radio',0,2));
	
	bag.set_token_count(Token['Skull'], lookupCount('skull-btn-radio',0,2));
	bag.set_token_value(Token['Skull'], parseInt(document.getElementById('skull-value').value));
	
	bag.set_token_count(Token['Cultist'], lookupCount('cultist-btn-radio',0,2));
	bag.set_token_value(Token['Cultist'], parseInt(document.getElementById('cultist-value').value));
	
	bag.set_token_count(Token['Tablet'], lookupCount('tablet-btn-radio',0,2));
	bag.set_token_value(Token['Tablet'], parseInt(document.getElementById('tablet-value').value));
	
	bag.set_token_count(Token['ElderThing'], lookupCount('elder-thing-btn-radio',0,2));
	bag.set_token_value(Token['ElderThing'], parseInt(document.getElementById('elder-thing-value').value));
	
	bag.set_token_count(Token['ElderSign'], lookupCount('elder-sign-btn-radio',0,1));
	bag.set_token_value(Token['ElderSign'], parseInt(document.getElementById('elder-sign-value').value));
	if (document.getElementById('elder-sign-auto-success-checkbox').checked) {
		bag.set_token_value(Token['ElderSign'], AutoSuccess);
	}
	
	bag.set_token_count(Token['AutoFail'], lookupCount('auto-fail-btn-radio',0,1));
	
	bag.set_token_count(Token['Bless'], lookupCount('bless-btn-radio',0,10));
	bag.set_token_count(Token['Curse'], lookupCount('curse-btn-radio',0,10));
	
	var probabilities = draw_bag(bag);
	buildPlot(probabilities);
}
