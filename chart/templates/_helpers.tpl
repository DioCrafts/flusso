# chart/templates/_helpers.tpl
{{- define "flusso-ingress-controller.fullname" -}}
{{- printf "%s-%s" .Release.Name .Chart.Name | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "flusso-ingress-controller.name" -}}
{{- .Release.Name | trunc 63 -}}
{{- end -}}

{{- define "flusso-ingress-controller.chart" -}}
{{- .Chart.Name -}}
{{- end -}}
