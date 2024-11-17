# chart/templates/_helpers.tpl
{{- define "api-gateway.fullname" -}}
{{- printf "%s-%s" .Release.Name .Chart.Name | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "api-gateway.name" -}}
{{- .Release.Name | trunc 63 -}}
{{- end -}}

{{- define "api-gateway.chart" -}}
{{- .Chart.Name -}}
{{- end -}}
